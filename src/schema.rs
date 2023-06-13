// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Integer,
        title -> Text,
        date -> Text,
        description -> Nullable<Text>,
    }
}
