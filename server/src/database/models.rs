use super::sqlite_uuid::UUID;
use diesel::prelude::*;
#[derive(Insertable, Queryable, Identifiable, Selectable, Debug, PartialEq, Clone)]
#[diesel(table_name = crate::database::schema::users)]
//#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: UUID,
    pub email: String,
    pub github_id: i32,
    pub github_access_token: String,
}

#[derive(Insertable, Queryable, Selectable, Associations, Debug, PartialEq)]
#[diesel(table_name = crate::database::schema::projects)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Project {
    pub id: UUID,
    pub name: String,
    pub user_id: UUID,
}

#[derive(Insertable, Queryable, Selectable, Associations, Debug, PartialEq)]
#[diesel(table_name = crate::database::schema::functions)]
#[diesel(belongs_to(Project))]
//#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Function {
    pub id: UUID,
    pub name: String,
    pub hash: String,
    pub project_id: UUID,
}
