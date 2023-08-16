// @generated automatically by Diesel CLI.

diesel::table! {
    functions (id) {
        id -> Text,
        name -> Text,
        hash -> Text,
        project_id -> Text,
    }
}

diesel::table! {
    projects (id) {
        id -> Text,
        name -> Text,
        user_id -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        email -> Text,
        github_id -> Integer,
        github_access_token -> Text,
    }
}

diesel::joinable!(functions -> projects (project_id));
diesel::joinable!(projects -> users (user_id));
diesel::allow_tables_to_appear_in_same_query!(functions, projects, users,);
