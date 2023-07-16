// @generated automatically by Diesel CLI.

diesel::table! {
    functions (id) {
        id -> Binary,
        name -> Text,
        hash -> Text,
        project_id -> Binary,
    }
}

diesel::table! {
    projects (id) {
        id -> Binary,
        name -> Text,
        user_id -> Binary,
    }
}

diesel::table! {
    users (id) {
        id -> Binary,
        email -> Text,
        github_id -> Integer,
        github_access_token -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    functions,
    projects,
    users,
);
