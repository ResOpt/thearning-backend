use crate::schema::users;
use crate::schema::students;
use crate::schema::teachers;

use crate::classes::models::*;
use crate::users::utils::*;
use crate::db;

use rocket_contrib::json::{Json, JsonValue};

use std::path::Path;

use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use serde::{Serialize,Deserialize};

use bcrypt::{DEFAULT_COST, hash, verify};
use std::fmt;

pub enum Role {
    Student,
    Teacher,
    Admin,
}

impl Role {
    pub fn from_str(role: &str) -> Result<Self, String> {
        match role {
            "admin" => Ok(Self::Admin),
            "teacher" => Ok(Self::Teacher),
            "student" => Ok(Self::Student),
            _ => Err("Invalid role".to_string()),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Teacher => write!(f, "Teacher"),
            Role::Admin => write!(f, "Admin"),
            Role::Student => write!(f, "Student"),
        }
    }
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub fullname: String,
    pub profile_photo: String,
    pub email: String,
    pub password: String,
    pub bio: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable)]
#[table_name = "students"]
pub struct Student {
    pub id: i32,
    pub student_id: String,
    pub class_id: String,
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable)]
#[table_name = "teachers"]
pub struct Teacher {
    pub id: i32,
    pub teacher_id: String,
    pub class_id: String,
}

impl User {

    pub fn create(user: Self, connection: &PgConnection) -> QueryResult<Self> {
        let hashed = Self {
            password: hash(user.password,DEFAULT_COST).unwrap(),
            ..user
        };
        diesel::insert_into(users::table)
            .values(&hashed)
            .execute(connection)?;

        users::table.order(users::id.desc()).first(connection)
    }

    pub fn get_by_key(key_: &String, password_: String, connection: &PgConnection) -> Option<Self> {
        let res;
        if is_email(key_) {
            res = users::table
                .filter(users::email.eq(key_))
                .get_result::<Self>(connection);
        }
        else {
            res = users::table
                .filter(users::id.eq(key_))
                .get_result::<Self>(connection);
        }
        match res {
            Ok(user) => {
                if let Ok(matching) = verify(&password_,&user.password) {
                    if matching {
                        return Some(user)
                    }
                }
                return None
            }
            Err(_) => {
                None
            }
        }
    }

    pub fn get_role(key_: &String, connection: &PgConnection) -> Result<Role, String> {
        let res;
        if is_email(key_) {
            res = users::table
                .filter(users::email.eq(key_))
                .get_result::<Self>(connection);
        }
        else {
            res = users::table
                .filter(users::id.eq(key_))
                .get_result::<Self>(connection);
        }
        match res {
            Ok(user) => Role::from_str(&user.status),
            Err(e) => Err("User does not exist".to_string()),
        }
    }
}

impl Student {
    pub fn create(uid: String, class_id: String, connection: &PgConnection) -> QueryResult<Self> {
        let x = Self {
            id: generate_random_id(),
            student_id: uid,
            class_id: class_id,
        };
        diesel::insert_into(students::table)
            .values(&x)
            .execute(connection)?;
            // .map(|_| Json(json!({"success": true, "status": "teacher"})))
            // .map_err(|e| Json(json!({"success":false, "message":e.to_string()})))
        students::table.order(students::student_id.desc()).first(connection)

    }
}

impl Teacher {
    pub fn create(uid: String, class_id: String, connection: &PgConnection) -> QueryResult<Self> {
        let x = Self {
            id: generate_random_id(),
            teacher_id: uid,
            class_id: class_id,
        };
        diesel::insert_into(teachers::table)
            .values(&x)
            .execute(connection)?;
            // .map(|_| Json(json!({"success": true, "status": "teacher"})))
            // .map_err(|e| Json(json!({"success":false, "message":e.to_string()})))
        teachers::table.order(teachers::teacher_id.desc()).first(connection)

    }
}