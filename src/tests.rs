#[cfg(test)]
mod tests {
    extern crate diesel;

    use rocket::http::{ContentType, Header, Status};
    use rocket::local::blocking::Client;
    use rocket::serde::Deserialize;

    use crate::assignments::models::Assignment;
    use crate::attachments::models::Attachment;
    use crate::auth::read_token;
    use crate::classes::models::Classroom;
    use crate::db::database_url;
    use crate::files::models::UploadedFile;
    use crate::links::models::Link;
    use crate::rocket;
    use crate::schema::assignments::dsl::assignments as assignment_object;
    use crate::schema::classes;
    use crate::schema::classes::dsl::classes as classes_object;
    use crate::schema::files::dsl::files as files_object;
    use crate::schema::students::dsl::students as students_object;
    use crate::schema::teachers::dsl::teachers as teachers_object;
    use crate::schema::users;
    use crate::schema::users::dsl::users as users_object;
    use crate::submissions::models::Submissions;
    use crate::traits::ClassUser;
    use crate::users::models::Student;

    use self::diesel::prelude::*;

    #[derive(Deserialize)]
    struct Auth {
        status: i32,
        token: String,
    }

    #[derive(Deserialize)]
    struct ClassId {
        class_id: String,
    }

    #[derive(Deserialize)]
    struct ClassIds {
        class_ids: Vec<Classroom>,
    }

    #[derive(Deserialize)]
    struct SuccessOrNot {
        success: bool,
    }

    #[derive(Deserialize)]
    struct AssignmentId {
        assignment_id: String,
    }

    #[derive(Deserialize)]
    struct AssignmentName {
        assignment_name: String,
    }

    #[derive(Deserialize)]
    struct AttachmentData {
        attachment: Option<Attachment>,
        file: Option<UploadedFile>,
        link: Option<Link>,
    }

    #[derive(Deserialize)]
    struct AssignmentData {
        assignment: Assignment,
        submission: Submissions,
        assignment_attachments: Vec<AttachmentData>,
    }

    #[derive(Deserialize)]
    struct NewAssignment {
        new_assignment: AssignmentName,
    }

    #[derive(Deserialize)]
    struct ClassResp {
        assignments: Vec<Assignment>,
    }

    #[derive(Deserialize)]
    struct UserData {
        user_id: String,
        fullname: String,
        email: String,
        status: String,
        profile_photo: String,
        bio: String,
    }

    #[derive(Deserialize)]
    struct UserDataResponse {
        status: i32,
        data: UserData,
    }

    fn client() -> Client {
        Client::tracked(rocket()).expect("valid rocket instance")
    }

    fn auth_request() -> (Auth, Auth) {
        // Construct the client
        let client = client();

        // Authenticating the dummy users
        let mut response_auth = client
            .post("/api/auth")
            .header(ContentType::JSON)
            .body(r#"{"key":"123", "password":"dummy"}"#)
            .dispatch();

        let mut response_auth_2 = client
            .post("/api/auth")
            .header(ContentType::JSON)
            .body(r#"{"key":"234", "password":"dummy"}"#)
            .dispatch();

        (
            response_auth.into_json::<Auth>().unwrap(),
            response_auth_2.into_json::<Auth>().unwrap(),
        )
    }

    #[test]
    fn t_1_create_user() {
        let string = "user_id=123&fullname=Dummy Student&image=0/placeholder.png&file_name=placeholder.png&email=dummystudent@mail.com&password=dummy&bio=Dummy&status=student&birth_place=Indonesia&birth_date=2005-01-01";

        let string_2 = "user_id=234&fullname=Dummy Teacher&image=0&file_name=placeholder.png&email=dummyteacher@mail.com&password=dummy&bio=Dummy&status=teacher&birth_place=Indonesia&birth_date=1990-01-01";

        // Construct the client
        let client = client();

        // Creating the dummy users
        let response_create = client
            .post("/api/user")
            //.header(ContentType::new("multipart", "form-data; boundary=--------------------------293582696224464"))
            .header(ContentType::Form)
            .body(string)
            .dispatch();

        let response_create_2 = client
            .post("/api/user")
            //.header(ContentType::new("multipart", "form-data; boundary=--------------------------293582696224464"))
            .header(ContentType::Form)
            .body(string_2)
            .dispatch();

        // Is Response ok?
        assert_eq!(response_create.status(), Status::Ok);
        assert_eq!(response_create_2.status(), Status::Ok);
    }

    #[test]
    fn t_2_auth() {
        // Getting the auth response
        let r = auth_request();

        // Are status' equal to 200?
        assert_eq!(r.0.status, 200);
        assert_eq!(r.1.status, 200);
    }

    #[test]
    fn t_3_get_data() {
        // Construct the client
        let client = client();

        // Getting the auth response
        let r = auth_request();

        // Sending GET method to get data with token authentication
        let mut response_data = client
            .get("/api/user")
            .header(Header::new(
                "Authorization",
                format!("Bearer {}", r.0.token),
            ))
            .dispatch();

        let mut response_data_2 = client
            .get("/api/user")
            .header(Header::new(
                "Authorization",
                format!("Bearer {}", r.1.token),
            ))
            .dispatch();

        // Deserializing data
        let r_2 = response_data.into_json::<UserDataResponse>().unwrap();
        let r_3 = response_data_2.into_json::<UserDataResponse>().unwrap();

        // Is user_id equals to 123?
        assert_eq!(r_2.data.user_id, "123");
        assert_eq!(r_3.data.user_id, "234");
    }

    #[test]
    fn t_4_create_classroom() {
        let r = auth_request();

        let read = read_token(&r.0.token).unwrap();

        let string = format!(
            "class_name=Test Class&class_description=Just a Test Class&section=Testing Class"
        );

        let client = client();

        // Sending POST method to create a classroom
        let mut response_classroom = client
            .post("/api/classroom")
            .header(Header::new(
                "Authorization",
                format!("Bearer {}", r.1.token),
            ))
            .header(ContentType::Form)
            .body(string)
            .dispatch();

        // Deserializing Classroom reponse
        let r = response_classroom.into_json::<ClassId>().unwrap();

        // Is it success?
        assert_eq!(!r.class_id.is_empty(), true);
    }

    #[test]
    fn t_5_create_assignment() {
        let client = client();

        let token = auth_request().1.token;

        // Sending get request to get classrooms the user attends
        let response_1 = client
            .get("/api/classroom")
            .header(Header::new(
                "Authorization",
                format!("Bearer {}", token.clone()),
            ))
            .dispatch();

        // Deserialize it into a struct
        let r = response_1.into_json::<ClassIds>().unwrap();

        let classrooms = r.class_ids;

        // Getting a sample class id
        let sample_class = classrooms.first().unwrap();

        // Sending post request to create a new assignment
        let response_2 = client
            .post(format!(
                "/api/classroom/{}/assignments",
                &sample_class.class_id
            ))
            .header(ContentType::JSON)
            .header(Header::new(
                "Authorization",
                format!("Bearer {}", token.clone()),
            ))
            .dispatch();

        // Deserializing the response
        let r_2 = response_2.into_json::<AssignmentId>().unwrap();

        // Was the operation successful?
        assert_ne!(r_2.assignment_id, "".to_string());

        let string = format!(
            r#"{{
                "id": "{}",
                "assignment": {{"assignment_name": "Dummy Assignment",
                                                       "class_id": "{}",
                                                       "due_date": null,
                                                       "due_time": null,
                                                       "instructions": "Do this and that"
                                                     }},
                                         "files": null
                                  }}"#,
            r_2.assignment_id, &sample_class.class_id
        );

        let response_3 = client
            .patch(format!(
                "/api/classroom/{}/assignments",
                &sample_class.class_id
            ))
            .header(ContentType::JSON)
            .header(Header::new(
                "Authorization",
                format!("Bearer {}", token.clone()),
            ))
            .body(string)
            .dispatch();

        // Deserializing the response
        let r_3 = response_3.into_json::<NewAssignment>().unwrap();

        // Was the operation successful?
        assert_eq!(
            r_3.new_assignment.assignment_name,
            "Dummy Assignment".to_string()
        );
    }

    #[test]
    fn t_6_join_classroom() {
        let client = client();

        let teacher_token = auth_request().1.token;

        let token = auth_request().0.token;

        // Sending get request to get classrooms the user attends
        let response_1 = client
            .get("/api/classroom")
            .header(Header::new(
                "Authorization",
                format!("Bearer {}", teacher_token),
            ))
            .dispatch();

        // Deserialize it into a struct
        let r = response_1.into_json::<ClassIds>().unwrap();

        let classrooms = r.class_ids;

        // Getting a sample class id
        let sample_class = classrooms.first().unwrap();

        let class_id = &sample_class.class_id;

        let response_2 = client
            .post(format!("/api/classroom/{}", class_id))
            .header(Header::new(
                "Authorization",
                format!("Bearer {}", token.clone()),
            ))
            .dispatch();

        // Is the student creation success?
        assert_eq!(response_2.status(), Status::Ok);
    }

    #[test]
    fn t_7_get_student_assignments() {
        let db_conn = PgConnection::establish(&database_url()).unwrap();

        let client = client();

        let token = auth_request().0.token;

        // Sending get request to get classrooms the user attends
        let response_1 = client
            .get("/api/classroom")
            .header(Header::new("Authorization", format!("Bearer {}", &token)))
            .dispatch();

        // Deserialize it into a struct
        let r = response_1.into_json::<ClassIds>().unwrap();

        let classrooms = r.class_ids;

        // Getting a sample class id
        let sample_class = classrooms.first().unwrap();

        // Sending post request to create a new assignment
        let response_2 = client
            .get(format!("/api/classroom/{}", &sample_class.class_id))
            .header(ContentType::JSON)
            .header(Header::new("Authorization", format!("Bearer {}", &token)))
            .dispatch();

        // Deserializing the response
        let r_2 = response_2.into_json::<ClassResp>().unwrap();

        let assignment = r_2.assignments.first().unwrap();

        let submission =
            Submissions::get_by_assignment(&assignment.assignment_id, &db_conn).unwrap();

        // Sending get request to get classrooms the user attends
        let response_3 = client
            .get(format!(
                "/api/classroom/{}/assignments/students/{}",
                &sample_class.class_id, &assignment.assignment_id
            ))
            .header(Header::new("Authorization", format!("Bearer {}", &token)))
            .dispatch();

        let r_3 = response_3.into_json::<AssignmentData>().unwrap();

        assert_eq!(r_3.assignment.assignment_id, assignment.assignment_id);
        assert_eq!(r_3.submission.submission_id, submission.submission_id);
        assert_eq!(r_3.submission.user_id, read_token(&token).unwrap());
    }

    #[test]
    fn t_8_update_user() {
        let db_conn = PgConnection::establish(&database_url()).unwrap();

        let auth = auth_request();

        let auth_2 = auth_request();

        let string = "fullname=Dummy Student Edited&email=dummystudentedited@mail.com&bio=This user is edited&status=student&birth_place=Indonesia&birth_date=2005-01-01";

        let client = client();

        let response = client
            .post("/api/user/update")
            .header(Header::new(
                "Authorization",
                format!("Bearer {}", auth.0.token),
            ))
            .header(ContentType::Form)
            .body(string)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);

        let mut get_request = client
            .get("/api/user")
            .header(Header::new(
                "Authorization",
                format!("Bearer {}", auth.0.token),
            ))
            .dispatch();

        let r = get_request.into_json::<UserDataResponse>().unwrap();

        assert_eq!(r.status, 200);
        assert_eq!(r.data.fullname, "Dummy Student Edited");
    }

    #[test]
    fn t_9_delete_all() {
        // Database connection
        let db_conn = PgConnection::establish(&database_url()).unwrap();

        let delete_all_files = diesel::delete(files_object).execute(&db_conn);

        // Deleting all assignments in the table
        let delete_all_assignments = diesel::delete(assignment_object).execute(&db_conn);

        // Deleting all students in the table
        let delete_all_students = diesel::delete(students_object).execute(&db_conn);

        // Deleting all teachers in the table
        let delete_all_teachers = diesel::delete(teachers_object).execute(&db_conn);

        // Deleting all classes in the table
        let delete_all_classes = diesel::delete(classes_object).execute(&db_conn);

        // Deleting the dummy users
        let delete_dummy_1 =
            diesel::delete(users_object.filter(users::user_id.eq("123"))).execute(&db_conn);

        let delete_dummy_2 =
            diesel::delete(users_object.filter(users::user_id.eq("234"))).execute(&db_conn);

        // Are the rows deleted?
        assert_eq!(Ok(2), delete_all_files);
        assert_eq!(Ok(1), delete_all_assignments);
        assert_eq!(Ok(1), delete_all_students);
        assert_eq!(Ok(1), delete_all_teachers);
        assert_eq!(Ok(1), delete_all_classes);
        assert_eq!(Ok(1), delete_dummy_1);
        assert_eq!(Ok(1), delete_dummy_2);
    }
}
