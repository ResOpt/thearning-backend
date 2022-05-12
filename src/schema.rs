table! {
    admins (id) {
        id -> Int4,
        user_id -> Varchar,
        class_id -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    announcements (announcement_id) {
        announcement_id -> Varchar,
        announcement_name -> Nullable<Varchar>,
        class_id -> Varchar,
        posted_date -> Date,
        body -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

table! {
    assignments (assignment_id) {
        assignment_id -> Varchar,
        assignment_name -> Nullable<Varchar>,
        class_id -> Nullable<Varchar>,
        topic_id -> Nullable<Varchar>,
        due_date -> Nullable<Date>,
        due_time -> Nullable<Time>,
        posted_date -> Date,
        instructions -> Nullable<Text>,
        total_marks -> Nullable<Int4>,
        created_at -> Timestamp,
        creator -> Nullable<Varchar>,
        draft -> Bool,
    }
}

table! {
    attachments (attachment_id) {
        attachment_id -> Varchar,
        file_id -> Nullable<Varchar>,
        link_id -> Nullable<Varchar>,
        assignment_id -> Nullable<Varchar>,
        announcement_id -> Nullable<Varchar>,
        submission_id -> Nullable<Varchar>,
        uploader -> Varchar,
        created_at -> Timestamp,
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
        created_at -> Timestamp,
    }
}

table! {
    comments (id) {
        id -> Varchar,
        user_id -> Varchar,
        assignment_id -> Nullable<Varchar>,
        announcement_id -> Nullable<Varchar>,
        body -> Text,
        created_at -> Timestamp,
    }
}

table! {
    files (file_id) {
        file_id -> Varchar,
        filename -> Varchar,
        file_path -> Varchar,
        file_url -> Varchar,
        filetype -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    links (id) {
        id -> Varchar,
        title -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        thumbnail -> Nullable<Varchar>,
        url -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

table! {
    private_comments (id) {
        id -> Varchar,
        user_id -> Varchar,
        submission_id -> Nullable<Varchar>,
        body -> Text,
        created_at -> Timestamp,
    }
}

table! {
    students (id) {
        id -> Int4,
        user_id -> Varchar,
        class_id -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    submissions (submission_id) {
        submission_id -> Varchar,
        assignment_id -> Varchar,
        user_id -> Varchar,
        submitted_date -> Nullable<Date>,
        submitted_time -> Nullable<Time>,
        on_time -> Nullable<Bool>,
        marks_allotted -> Nullable<Int4>,
        submitted -> Bool,
        created_at -> Timestamp,
    }
}

table! {
    teachers (id) {
        id -> Int4,
        user_id -> Varchar,
        class_id -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    topics (id) {
        id -> Varchar,
        topic_name -> Varchar,
        classroom_id -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    users (user_id) {
        user_id -> Varchar,
        fullname -> Varchar,
        profile_photo -> Varchar,
        email -> Varchar,
        password -> Varchar,
        birth_place -> Varchar,
        birth_date -> Date,
        bio -> Text,
        status -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(admins -> classes (class_id));
joinable!(admins -> users (user_id));
joinable!(announcements -> classes (class_id));
joinable!(assignments -> classes (class_id));
joinable!(assignments -> topics (topic_id));
joinable!(assignments -> users (creator));
joinable!(attachments -> announcements (announcement_id));
joinable!(attachments -> assignments (assignment_id));
joinable!(attachments -> files (file_id));
joinable!(attachments -> links (link_id));
joinable!(attachments -> submissions (submission_id));
joinable!(attachments -> users (uploader));
joinable!(classes -> users (class_creator));
joinable!(comments -> announcements (announcement_id));
joinable!(comments -> assignments (assignment_id));
joinable!(comments -> users (user_id));
joinable!(private_comments -> submissions (submission_id));
joinable!(private_comments -> users (user_id));
joinable!(students -> classes (class_id));
joinable!(students -> users (user_id));
joinable!(submissions -> assignments (assignment_id));
joinable!(submissions -> users (user_id));
joinable!(teachers -> classes (class_id));
joinable!(teachers -> users (user_id));
joinable!(topics -> classes (classroom_id));

allow_tables_to_appear_in_same_query!(
    admins,
    announcements,
    assignments,
    attachments,
    classes,
    comments,
    files,
    links,
    private_comments,
    students,
    submissions,
    teachers,
    topics,
    users,
);
