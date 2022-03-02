
#[cfg(test)]
mod test {

    extern crate diesel;

    use self::diesel::prelude::*;

    use std::io::Read;
    use rocket::http::{ContentType, Status};
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


    #[derive(Deserialize, Debug)]
    struct Task {
        status: i32,
        token: String,
    }

    #[test]
    fn create_and_auth_test() {

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

        // Authenticating the dummy user
        let mut response_auth = client.post("/api/auth")
            .header(ContentType::JSON)
            .body(r#"{"key":"123", "password":"dummy"}"#).dispatch();

        let r = response_auth.into_json::<Task>().unwrap();

        // Is status equals to 200?
        assert_eq!(r.status, 200);

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
