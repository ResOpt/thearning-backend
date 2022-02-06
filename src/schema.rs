table! {
    assignments (assignment_id) {
        assignment_id -> Varchar,
        assignment_name -> Varchar,
        class_id -> Varchar,
        due_date -> Nullable<Date>,
        due_time -> Nullable<Time>,
        posted_date -> Date,
        instructions -> Nullable<Text>,
        total_marks -> Nullable<Int4>,
    }
}

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
    submissions (submission_id) {
        submission_id -> Varchar,
        assignment_id -> Varchar,
        student_id -> Varchar,
        submitted_date -> Date,
        submitted_time -> Time,
        on_time -> Bool,
        marks_allotted -> Nullable<Int4>,
        submission_file -> Nullable<Varchar>,
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

joinable!(assignments -> classes (class_id));
joinable!(students -> users (student_id));
joinable!(submissions -> assignments (assignment_id));
joinable!(teachers -> users (teacher_id));

allow_tables_to_appear_in_same_query!(
    assignments,
    classes,
    students,
    submissions,
    teachers,
    users,
);
