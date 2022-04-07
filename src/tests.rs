#[cfg(test)]
mod tests {
    extern crate diesel;

    use rocket::http::{ContentType, Header, Status};
    use rocket::local::blocking::Client;
    use rocket::serde::Deserialize;

    use crate::auth::read_token;
    use crate::classes::models::Classroom;
    use crate::db::database_url;
    use crate::rocket;
    use crate::schema::assignments::dsl::assignments as assignment_object;
    use crate::schema::classes;
    use crate::schema::classes::dsl::classes as classes_object;
    use crate::schema::files::dsl::files as files_object;
    use crate::schema::students::dsl::students as students_object;
    use crate::schema::teachers::dsl::teachers as teachers_object;
    use crate::schema::users;
    use crate::schema::users::dsl::users as users_object;
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
        class_id: Vec<Classroom>,
    }

    #[derive(Deserialize)]
    struct SuccessOrNot {
        success: bool,
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
        let string = "user_id=123&fullname=Dummy Student&image=0/placeholder.png&file_name=placeholder.png&email=dummystudent@mail.com&password=dummy&bio=Dummy&status=student";

        let string_2 = "user_id=234&fullname=Dummy Teacher&image=0&file_name=placeholder.png&email=dummyteacher@mail.com&password=dummy&bio=Dummy&status=teacher";

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
            .header(Header::new("Authentication", r.0.token))
            .dispatch();

        let mut response_data_2 = client
            .get("/api/user")
            .header(Header::new("Authentication", r.1.token))
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

        let string = format!("class_name=Test Class&class_creator={}&class_description=Just a Test Class&section=Testing Class", read);

        let client = client();

        // Sending POST method to create a classroom
        let mut response_classroom = client
            .post("/api/classroom")
            .header(Header::new("Authentication", r.1.token))
            .header(ContentType::Form)
            .body(string)
            .dispatch();

        // Deserializing Classroom reponse
        let r = response_classroom.into_json::<ClassId>().unwrap();

        // Is it success?
        assert_eq!(!r.class_id.is_empty(), true);
    }

    #[test]
    fn t_5_join_classroom() {
        let db_conn = PgConnection::establish(&database_url()).unwrap();

        // Getting auth token
        let token = auth_request().0.token;

        // Loading classes
        let _classes = classes::table.load::<Classroom>(&db_conn).unwrap();

        // Loading a classroom and getting its id
        let class = _classes.first().unwrap();
        let class_id = &class.class_id;

        // Reading the auth token
        let user_id = read_token(&token).unwrap();

        // Creating student
        let create_student = Student::create(&user_id, class_id, &db_conn);

        // Is the student creation success?
        assert!(create_student.is_ok());
    }

    #[test]
    fn t_6_create_assignment() {
        let client = client();

        let token = auth_request().1.token;

        // Sending get request to get classrooms the user attends
        let response_1 = client
            .get("/api/classroom")
            .header(Header::new("Authentication", token.clone()))
            .dispatch();

        // Deserialize it into a struct
        let r = response_1.into_json::<ClassIds>().unwrap();

        let classrooms = r.class_id;

        // Getting a sample class id
        let sample_class = classrooms.first().unwrap();

        // Data
        let string = format!(
            r#"{{"assignment": {{"assignment_name": "Dummy Assignment",
                                                       "class_id": "{}",
                                                       "due_date": null,
                                                       "due_time": null,
                                                       "instructions": "Do this and that"
                                                     }},
                                         "files": null
                                  }}"#,
            sample_class.class_id
        );

        // Sending post request to create a new assignment
        let response_2 = client
            .post("/api/classroom/assignments/create")
            .header(ContentType::JSON)
            .header(Header::new("Authentication", token.clone()))
            .body(string)
            .dispatch();

        // Deserializing the response
        let r_2 = response_2.into_json::<SuccessOrNot>().unwrap();

        // Was the operation successful?
        assert_eq!(true, r_2.success)
    }

    #[test]
    fn t_7_delete_all() {
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
        assert_eq!(Ok(0), delete_all_files);
        assert_eq!(Ok(1), delete_all_assignments);
        assert_eq!(Ok(1), delete_all_students);
        assert_eq!(Ok(1), delete_all_teachers);
        assert_eq!(Ok(1), delete_all_classes);
        assert_eq!(Ok(1), delete_dummy_1);
        assert_eq!(Ok(1), delete_dummy_2);
    }
}
