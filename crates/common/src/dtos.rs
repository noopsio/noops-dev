use diesel::backend::Backend;
use diesel::{deserialize::FromSql, sql_types::Text};
use diesel::{
    serialize::{IsNull, Output, ToSql},
    *,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(
    AsExpression,
    FromSqlRow,
    Serialize,
    Deserialize,
    PartialEq,
    Debug,
    Clone,
    Default,
    Hash,
    Eq,
    Copy,
    PartialOrd,
    Ord,
)]
#[diesel(sql_type = Text)]
pub enum Language {
    #[default]
    #[serde(rename = "Rust")]
    Rust,
    #[serde(rename = "Golang")]
    Golang,
}

impl ToSql<Text, sqlite::Sqlite> for Language {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, sqlite::Sqlite>) -> serialize::Result {
        out.set_value(self.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, sqlite::Sqlite> for Language {
    fn from_sql(bytes: <sqlite::Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <String as FromSql<Text, sqlite::Sqlite>>::from_sql(bytes)?;

        match value.as_str() {
            "Rust" => Ok(Language::Rust),
            "Golang" => Ok(Language::Golang),
            _ => Err("Invalid language".into()),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct CreateFunctionDTO {
    pub name: String,
    pub language: Language,
    pub wasm: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct GetProjectDTO {
    pub name: String,
    pub handlers: Vec<GetHandlerDTO>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Default, Hash, PartialOrd, Ord)]
pub struct GetHandlerDTO {
    pub name: String,
    pub language: Language,
    pub hash: String,
    pub link: String,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Rust => f.write_str("Rust"),
            Language::Golang => f.write_str("Golang"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct GetJWTDTO {
    pub jwt: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct ErrorDTO {
    pub error_message: String,
}

impl ErrorDTO {
    pub fn new(error_message: &str) -> Self {
        Self {
            error_message: error_message.to_string(),
        }
    }
}
