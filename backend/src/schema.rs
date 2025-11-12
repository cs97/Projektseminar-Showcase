// @generated automatically by Diesel CLI.

diesel::table! {
    comment (id) {
        id -> Int4,
        title -> Text,
        body -> Text,
    }
}
