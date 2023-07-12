// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        email -> Text,
        github_id -> Integer,
        github_access_token -> Text,
    }
}
