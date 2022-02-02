table! {
    teachers (id) {
        id -> Int4,
        teacher_id -> Varchar,
        class_id -> Varchar,
    }
}

table! {
    users (id) {
        id -> Varchar,
        fullname -> Varchar,
        profile_photo -> Varchar,
        email -> Varchar,
        password -> Varchar,
        bio -> Text,
        status -> Varchar,
    }
}

joinable!(teachers -> users (teacher_id));

allow_tables_to_appear_in_same_query!(
    teachers,
    users,
);
