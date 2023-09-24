// @generated automatically by Diesel CLI.

diesel::table! {
    links (id) {
        id -> Text,
        url -> Text,
        count -> Integer,
    }
}
