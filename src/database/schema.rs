// @generated automatically by Diesel CLI.

diesel::table! {
    balance (id) {
        id -> Integer,
        person_id -> Integer,
        value -> Float,
    }
}

diesel::table! {
    movement (id) {
        id -> Integer,
        person_id -> Integer,
        value -> Float,
        actor -> Text,
        date -> Timestamp,
    }
}

diesel::table! {
    person (id) {
        id -> Integer,
        email -> Text,
        name -> Text,
        role -> Text,
        currency -> Text,
        dept -> Text,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        password -> Text,
        person_id -> Integer,
        superuser -> Bool,
        username -> Text,
    }
}

diesel::joinable!(balance -> person (person_id));
diesel::joinable!(movement -> person (person_id));
diesel::joinable!(user -> person (person_id));

diesel::allow_tables_to_appear_in_same_query!(balance, movement, person, user,);
