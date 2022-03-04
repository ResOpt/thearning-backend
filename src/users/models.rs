use std::fmt;
use std::path::Path;

use bcrypt::{DEFAULT_COST, hash, verify};
use diesel;
use diesel::associations::BelongsTo;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::classes::models::*;
use crate::db;
use crate::schema::admins;
use crate::schema::students;
use crate::schema::teachers;
use crate::schema::users;
use crate::users::utils::*;

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
            Role::Teacher => write!(f, "teacher"),
            Role::Admin => write!(f, "admin"),
            Role::Student => write!(f, "student"),
        }
    }
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Associations)]
#[table_name = "users"]
pub struct User {
    pub user_id: String,
    pub fullname: String,
    pub profile_photo: String,
    pub email: String,
    pub password: String,
    pub bio: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Associations, Identifiable, Debug)]
#[belongs_to(User)]
#[table_name = "students"]
pub struct Student {
    pub id: i32,
    pub user_id: String,
    pub class_id: String,
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Associations, Identifiable)]
#[belongs_to(User)]
#[table_name = "teachers"]
pub struct Teacher {
    pub id: i32,
    pub user_id: String,
    pub class_id: String,
}

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Associations, Identifiable)]
#[belongs_to(User)]
#[table_name = "admins"]
pub struct Admin {
    pub id: i32,
    pub user_id: String,
    pub class_id: String,
}

impl User {
    pub fn create(user: Self, connection: &PgConnection) -> QueryResult<Self> {
        let hashed = Self {
            password: hash(user.password, DEFAULT_COST).unwrap(),
            ..user
        };
        diesel::insert_into(users::table)
            .values(&hashed)
            .execute(connection)?;

        users::table.find(hashed.user_id).get_result::<Self>(connection)
    }

    pub fn find_user(uid: &String, connection: &PgConnection) -> QueryResult<Self> {
        users::table.find(uid).get_result::<Self>(connection)
    }

    pub fn get_by_key(key_: &String, password_: String, connection: &PgConnection) -> Option<Self> {
        let res = users::table
            .filter(users::user_id.eq(key_))
            .get_result::<Self>(connection);

        match res {
            Ok(user) => {
                if let Ok(matching) = verify(&password_, &user.password) {
                    if matching {
                        return Some(user);
                    }
                }
                return None;
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
        } else {
            res = users::table
                .filter(users::user_id.eq(key_))
                .get_result::<Self>(connection);
        }
        match res {
            Ok(user) => Role::from_str(&user.status),
            Err(e) => Err("User does not exist".to_string()),
        }
    }

    pub fn get_id_from_email(email: &String, connection: &PgConnection) -> Result<String, String> {
        let res = users::table
            .filter(users::email.eq(email))
            .get_result::<Self>(connection);

        match res {
            Ok(user) => Ok(user.user_id),
            Err(e) => Err("User does not exist".to_string()),
        }
    }
}

impl Student {
    pub fn create(uid: &String, class_id: &String, connection: &PgConnection) -> QueryResult<Self> {
        let new_student = Self {
            id: generate_random_id(),
            user_id: uid.to_string(),
            class_id: class_id.to_string(),
        };
        diesel::insert_into(students::table)
            .values(&new_student)
            .execute(connection)?;
        students::table.filter(students::user_id.eq(new_student.user_id)).get_result::<Self>(connection)
    }
}

impl Teacher {
    pub fn create(uid: &String, class_id: &String, connection: &PgConnection) -> QueryResult<Self> {
        let new_teacher = Self {
            id: generate_random_id(),
            user_id: uid.to_string(),
            class_id: class_id.to_string(),
        };
        diesel::insert_into(teachers::table)
            .values(&new_teacher)
            .execute(connection)?;
        teachers::table.filter(teachers::user_id.eq(new_teacher.user_id)).get_result::<Self>(connection)
    }
}

impl Admin {
    pub fn create(uid: &String, class_id: &String, connection: &PgConnection) -> QueryResult<Self> {
        let new_admin = Self {
            id: generate_random_id(),
            user_id: uid.to_string(),
            class_id: class_id.to_string(),
        };
        diesel::insert_into(admins::table)
            .values(&new_admin)
            .execute(connection)?;
        admins::table.filter(admins::user_id.eq(new_admin.user_id)).get_result::<Self>(connection)
    }
}
