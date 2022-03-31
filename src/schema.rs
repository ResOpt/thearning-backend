table! {
    admins (id) {
        id -> Int4,
        user_id -> Varchar,
        class_id -> Varchar,
    }
}

table! {
    assignments (assignment_id) {
        assignment_id -> Varchar,
        assignment_name -> Varchar,
        class_id -> Varchar,
        topic_id -> Nullable<Varchar>,
        due_date -> Nullable<Date>,
        due_time -> Nullable<Time>,
        posted_date -> Date,
        instructions -> Nullable<Text>,
        total_marks -> Nullable<Int4>,
    }
}

table! {
    attachments (attachment_id) {
        attachment_id -> Varchar,
        file_id -> Varchar,
        assignment_id -> Varchar,
        uploader -> Varchar,
    }
}

table! {
    classes (class_id) {
        class_id -> Varchar,
        class_name -> Varchar,
        class_creator -> Nullable<Varchar>,
        class_description -> Nullable<Varchar>,
        class_image -> Nullable<Varchar>,
        section -> Varchar,
    }
}

table! {
    files (file_id) {
        file_id -> Varchar,
        filename -> Varchar,
        file_path -> Varchar,
        file_url -> Varchar,
        filetype -> Varchar,
    }
}

table! {
    students (id) {
        id -> Int4,
        user_id -> Varchar,
        class_id -> Varchar,
    }
}

table! {
    submissions (submission_id) {
        submission_id -> Varchar,
        assignment_id -> Varchar,
        user_id -> Varchar,
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
        user_id -> Varchar,
        class_id -> Varchar,
    }
}

table! {
    topics (id) {
        id -> Varchar,
        topic_name -> Varchar,
        classroom_id -> Varchar,
    }
}

table! {
    users (user_id) {
        user_id -> Varchar,
        fullname -> Varchar,
        profile_photo -> Varchar,
        email -> Varchar,
        password -> Varchar,
        bio -> Text,
        status -> Varchar,
    }
}

joinable!(admins -> classes (class_id));
joinable!(admins -> users (user_id));
joinable!(assignments -> classes (class_id));
joinable!(assignments -> topics (topic_id));
joinable!(attachments -> assignments (assignment_id));
joinable!(attachments -> files (file_id));
joinable!(attachments -> users (uploader));
joinable!(classes -> users (class_creator));
joinable!(students -> classes (class_id));
joinable!(students -> users (user_id));
joinable!(submissions -> assignments (assignment_id));
joinable!(submissions -> users (user_id));
joinable!(teachers -> classes (class_id));
joinable!(teachers -> users (user_id));
joinable!(topics -> classes (classroom_id));

allow_tables_to_appear_in_same_query!(
    admins,
    assignments,
    attachments,
    classes,
    files,
    students,
    submissions,
    teachers,
    topics,
    users,
);
