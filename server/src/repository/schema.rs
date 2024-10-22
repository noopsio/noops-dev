// @generated automatically by Diesel CLI.

diesel::table! {
    handlers (id) {
        id -> Text,
        name -> Text,
        language -> Text,
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
        name -> Nullable<Text>,
        location -> Nullable<Text>,
        company -> Nullable<Text>,
        github_login -> Text,
        github_id -> Integer,
        github_access_token -> Text,
    }
}

diesel::joinable!(handlers -> projects (project_id));
diesel::joinable!(projects -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(handlers, projects, users,);
