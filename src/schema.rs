table! {
    outcomes (id) {
        id -> Nullable<Integer>,
        title -> Text,
        description -> Nullable<Text>,
        creation_date -> Timestamp,
        resolution_date -> Timestamp,
    }
}

table! {
    prediction_events (id) {
        id -> Nullable<Integer>,
        by_user -> Integer,
        for_outcome -> Integer,
        prediction -> Bool,
        creation_date -> Timestamp,
    }
}

table! {
    transactions (id) {
        id -> Nullable<Integer>,
        date -> Timestamp,
        amount -> Integer,
        user_id -> Integer,
    }
}

table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        display_name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    outcomes,
    prediction_events,
    transactions,
    users,
);
