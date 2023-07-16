use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Binary;
use diesel::sqlite::Sqlite;
use diesel::{AsExpression, FromSqlRow};
use std::fmt;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

use crate::errors::Error;

#[derive(Debug, Clone, Copy, FromSqlRow, AsExpression, Hash, Eq, PartialEq)]
#[sql_type = "Binary"]
pub struct UUID(pub uuid::Uuid);

impl UUID {
    pub fn new() -> Self {
        let node_id: [u8; 6] = [1, 2, 3, 4, 5, 6];
        Self(Uuid::now_v1(&node_id))
    }

    pub fn from_str(uuid: &str) -> Result<Self, Error> {
        let uuid = Uuid::parse_str(uuid).map_err(|err| anyhow::anyhow!(err))?;
        Ok(Self(uuid))
    }
}

impl From<UUID> for uuid::Uuid {
    fn from(s: UUID) -> Self {
        s.0
    }
}

impl Display for UUID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromSql<Binary, Sqlite> for UUID {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let bytes = <Vec<u8> as FromSql<Binary, Sqlite>>::from_sql(bytes)?;
        Ok(UUID(Uuid::from_slice(&bytes)?))
    }
}

impl ToSql<Binary, Sqlite> for UUID {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let bytes = self.0.as_bytes();
        <[u8] as ToSql<Binary, Sqlite>>::to_sql(bytes, out)
    }
}
