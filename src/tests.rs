#[cfg(test)]
mod tests {
    extern crate diesel;

    use self::diesel::prelude::*;

    use std::io::Read;
    use rocket::http::{ContentType, Status, Header};
    use rocket::local::blocking::Client;
    use rocket::serde::json::Json;
    use rocket::serde::json::serde_json::json;
    use rocket::serde::Deserialize;

    use rustc_serialize::json::ToJson;
    use rustc_serialize::json::Json as EnumJson;
    use crate::classes::models::Classroom;

    use crate::rocket;
    use crate::schema::users::status;
    use crate::schema::users::dsl::*;
    use crate::db::{self, database_url};
    use crate::schema::classes::dsl::classes;
    use crate::schema::teachers::dsl::teachers;

    #[derive(Deserialize)]
    struct Task {
        status: i32,
        token: String,
    }

    #[derive(Deserialize)]
    struct Task2 {
        class_id: String,
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

    fn auth_request() -> (Task, Task) {

        // Construct the client
        let client = client();

        // Authenticating the dummy users
        let mut response_auth = client.post("/api/auth")
            .header(ContentType::JSON)
            .body(r#"{"key":"123", "password":"dummy"}"#).dispatch();

        let mut response_auth_2 = client.post("/api/auth")
            .header(ContentType::JSON)
            .body(r#"{"key":"234", "password":"dummy"}"#).dispatch();

        (response_auth.into_json::<Task>().unwrap(), response_auth_2.into_json::<Task>().unwrap())
    }

    #[test]
    fn t_1_create_user() {
        let string = r#"{
                         "user_id": "123", 
                         "fullname":"Dummy Student",
                         "profile_photo":"dummy.jpg", 
                         "email":"dummystudent@mail.com",
                         "password":"dummy", 
                         "bio": "Dummy", 
                         "status":"student"
                        }
                     "#;

        let string_2 = r#"{
                         "user_id": "234",
                         "fullname":"Dummy Teacher",
                         "profile_photo":"dummy.jpg",
                         "email":"dummyteacher@mail.com",
                         "password":"dummy",
                         "bio": "Dummy",
                         "status":"teacher"
                        }
                     "#;

        // Construct the client
        let client = client();

        // Creating the dummy users
        let response_create = client.post("/api/user")
            .header(ContentType::JSON)
            .body(string).dispatch();

        let response_create_2 = client.post("/api/user")
            .header(ContentType::JSON)
            .body(string_2).dispatch();

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
        let mut response_data = client.get("/api/user")
            .header(Header::new("Authentication", r.0.token))
            .dispatch();

        let mut response_data_2 = client.get("/api/user")
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
        let string = r#"
                    {
                        "class_name":"Dummy Class",
                        "section":"Dummy Section"
                    }
                 "#;

        let client = client();

        let r = auth_request();

        // Sending POST method to create a classroom
        let mut response_classroom = client.post("/api/classroom")
            .header(Header::new("Authentication", r.1.token))
            .header(ContentType::JSON)
            .body(string).dispatch();

        // Deserializing Classroom reponse
        let r = response_classroom.into_json::<Task2>().unwrap();

        // Is it success?
        assert_eq!(r.success, true);
    }

    #[test]
    fn t_5_delete_user() {

        // Database connection
        let db_conn = PgConnection::establish(&database_url()).unwrap();

        // Deleting all teachers in the table
        let delete_all_teachers = diesel::delete(teachers)
            .execute(&db_conn);

        // Deleting all classes in the table
        let delete_all_classes = diesel::delete(classes)
            .execute(&db_conn);

        // Deleting the dummy users
        let deleted_rows = diesel::delete(users
            .filter(user_id.eq("123")))
            .execute(&db_conn);

        let deleted_rows_2 = diesel::delete(users
            .filter(user_id.eq("234")))
            .execute(&db_conn);

        // Are the rows deleted?
        assert_eq!(Ok(1), delete_all_teachers);
        assert_eq!(Ok(1), delete_all_classes);
        assert_eq!(Ok(1), deleted_rows);
        assert_eq!(Ok(1), deleted_rows_2);
    }
}
