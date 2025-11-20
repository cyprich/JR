// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        priority -> Integer,
        planned_from -> Text,
        planned_duration -> Integer,
        real_from -> Nullable<Text>,
        real_duration -> Nullable<Integer>,
    }
}
