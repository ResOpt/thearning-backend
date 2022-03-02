
#[cfg(test)]
mod test {

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

    use crate::rocket;
    use crate::schema::users::status;
    use crate::schema::users::dsl::*;
    use crate::db::{self, database_url};


    #[derive(Deserialize)]
    struct Task {
        status: i32,
        token: String,
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

    fn auth_request() -> Task {

        // Construct the client
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        // Authenticating the dummy user
        let mut response_auth = client.post("/api/auth")
            .header(ContentType::JSON)
            .body(r#"{"key":"123", "password":"dummy"}"#).dispatch();

        response_auth.into_json::<Task>().unwrap()

    }

    #[test]
    fn t_1_create_user() {

        let string = r#"{
                         "user_id": "123", 
                         "fullname":"Dummy", 
                         "profile_photo":"dummy.jpg", 
                         "email":"dummy@mail.com", 
                         "password":"dummy", 
                         "bio": "Dummy", 
                         "status":"student"
                        }
                     "#;

        // Construct the client
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        // Creating the dummy user
        let response_create = client.post("/api/user")
            .header(ContentType::JSON)
            .body(string).dispatch();

        // Is Response ok?
        assert_eq!(response_create.status(), Status::Ok);

    }

    #[test]
    fn t_2_auth() {
        
        // Getting the auth response
        let r = auth_request();

        // Is status equals to 200?
        assert_eq!(r.status, 200);

    }

    #[test]
    fn t_3_get_data() {
        
        // Construct the client
        let client = Client::tracked(rocket()).expect("valid rocket instance");
       
        // Getting the auth response
        let r = auth_request();

        // Sending GET method to get data with token authentication
        let mut response_data = client.get("/api/user")
            .header(Header::new("Authentication", r.token))
            .dispatch();

        // Deserializing data
        let r_2 = response_data.into_json::<UserDataResponse>().unwrap();

        // Is user_id equals to 123?
        assert_eq!(r_2.data.user_id, "123");

    }

    #[test]
    fn t_4_delete_user() {

        // Database connection
        let db_conn = PgConnection::establish(&database_url()).unwrap();

        // Deleting the dummy user
        let deleted_rows = diesel::delete(users
            .filter(user_id.eq("123")))
            .execute(&db_conn);

        // Is the row deleted?
        assert_eq!(Ok(1), deleted_rows);

    }

}
