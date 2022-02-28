
#[cfg(test)]
mod test {
    use std::io::Read;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;
    use rocket::serde::json::Json;
    use rocket::serde::json::serde_json::json;
    use rustc_serialize::json::ToJson;
    use rustc_serialize::json::Json as EnumJson;
    use crate::rocket;
    use crate::schema::users::status;
    use rocket::serde::Deserialize;
//    assert_eq!(response.body() ,200);

    #[derive(Deserialize, Debug)]
    struct Task {
        status: i32,
        token: String,
    }

    #[test]
    fn create_and_auth_test() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let mut response_create = client.post("/api/user")
            .header(ContentType::JSON)
            .body(r#"{"user_id": "123", "fullname":"Dummy", "profile_photo":"dummy.jpg", "email":"dummy@mail.com", "password":"dummy", "bio": "Dummy", "status":"student"}"#).dispatch();

        assert_eq!(response_create.status(), Status::Ok);

        let mut response_auth = client.post("/api/auth")
            .header(ContentType::JSON)
            .body(r#"{"key":"123", "password":"dummy"}"#).dispatch();

        let r = response_auth.into_json::<Task>().unwrap();

        assert_eq!(r.status, 200);
    }

}