table! {
    classes (class_id) {
        class_id -> Varchar,
        class_name -> Varchar,
        section -> Varchar,
    }
}

table! {
    students (id) {
        id -> Int4,
        student_id -> Varchar,
        class_id -> Varchar,
    }
}

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

joinable!(students -> users (student_id));
joinable!(teachers -> users (teacher_id));

allow_tables_to_appear_in_same_query!(
    classes,
    students,
    teachers,
    users,
);
