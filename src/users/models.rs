use std::fmt;

use crate::errors::{ErrorKind, ThearningResult};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Local, NaiveDate, NaiveDateTime};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket::fs::TempFile;
use serde::{Deserialize, Serialize};

use crate::schema::admins;
use crate::schema::students;
use crate::schema::teachers;
use crate::schema::users;
use crate::traits::{ClassUser, Manipulable};
use crate::utils::{generate_random_id, NaiveDateForm};

pub enum Role {
    Student,
    Teacher,
    Admin,
}

impl From<&str> for Role {
    fn from(role: &str) -> Self {
        match role {
            "admin" => Self::Admin,
            "teacher" => Self::Teacher,
            "student" => Self::Student,
            _ => unimplemented!(),
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

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Associations, Clone)]
#[table_name = "users"]
pub struct User {
    pub user_id: String,
    pub fullname: String,
    pub profile_photo: String,
    pub email: String,
    pub password: String,
    pub birth_place: String,
    pub birth_date: NaiveDate,
    pub bio: String,
    pub status: String,
    pub created_at: NaiveDateTime,
}

#[derive(FromForm)]
pub struct InsertableUser<'a> {
    pub user_id: String,
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub birth_place: String,
    pub birth_date: NaiveDateForm,
    pub bio: String,
    pub status: String,
    pub image: Option<TempFile<'a>>,
    pub file_name: Option<String>,
}

#[derive(FromForm)]
pub struct UpdatableUser<'a> {
    pub fullname: String,
    pub email: String,
    pub birth_place: String,
    pub birth_date: NaiveDateForm,
    pub bio: String,
    pub image: Option<TempFile<'a>>,
    pub file_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PasswordChange<'a> {
    pub user_id: &'a str,
    pub password: &'a str,
    pub new_password: &'a str,
}

#[derive(
    Serialize, Deserialize, Queryable, AsChangeset, Insertable, Associations, Identifiable, Debug, Clone
)]
#[belongs_to(User)]
#[table_name = "students"]
pub struct Student {
    pub id: i32,
    pub user_id: String,
    pub class_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(
    Serialize, Deserialize, Queryable, AsChangeset, Insertable, Associations, Identifiable,
)]
#[belongs_to(User)]
#[table_name = "teachers"]
pub struct Teacher {
    pub id: i32,
    pub user_id: String,
    pub class_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(
    Serialize, Deserialize, Queryable, AsChangeset, Insertable, Associations, Identifiable,
)]
#[belongs_to(User)]
#[table_name = "admins"]
pub struct Admin {
    pub id: i32,
    pub user_id: String,
    pub class_id: String,
    pub created_at: NaiveDateTime,
}

impl User {
    pub fn find_user(uid: &String, connection: &PgConnection) -> ThearningResult<Self> {
        let res = users::table.find(uid).get_result::<Self>(connection)?;

        Ok(res)
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
            Err(_) => None,
        }
    }

    pub fn get_role(key_: &String, connection: &PgConnection) -> ThearningResult<Role> {
        let res = users::table
            .filter(users::user_id.eq(key_))
            .get_result::<Self>(connection)?;

        Ok(Role::from(res.status.as_str()))
    }

    pub fn get_id_from_email(email: &String, connection: &PgConnection) -> ThearningResult<String> {
        let res = users::table
            .filter(users::email.eq(email))
            .get_result::<Self>(connection);

        match res {
            Ok(user) => Ok(user.user_id),
            Err(e) => Err(ErrorKind::from(e)),
        }
    }

    pub fn update_password(&self, data: PasswordChange, conn: &PgConnection) -> Result<(), ()> {
        let new_hashed = hash(&data.new_password, DEFAULT_COST).unwrap();

        match verify(&data.password, &self.password) {
            Ok(matching) => {
                if matching {
                    diesel::update(users::table.filter(users::user_id.eq(&self.user_id)))
                        .set(users::password.eq(&new_hashed))
                        .execute(conn)
                        .unwrap();
                    Ok(())
                } else {
                    Err(())
                }
            }
            Err(_) => {
                return Err(());
            }
        }
    }

    pub fn is_admin(&self) -> bool {
        self.status == "admin"
    }

    pub fn is_teacher(&self) -> bool {
        self.status == "teacher"
    }

    pub fn is_student(&self) -> bool {
        self.status == "student"
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResponseUser {
    pub user_id: String,
    pub fullname: String,
    pub profile_photo: String,
    pub email: String,
    pub birth_place: String,
    pub birth_date: NaiveDate,
    pub bio: String,
    pub status: String,
    pub created_at: NaiveDateTime,
}

impl From<User> for ResponseUser {
    fn from(data: User) -> Self {
        Self {
            user_id: data.user_id,
            fullname: data.fullname,
            profile_photo: data.profile_photo,
            email: data.email,
            birth_place: data.birth_place,
            birth_date: data.birth_date,
            bio: data.bio,
            status: data.status,
            created_at: data.created_at,
        }
    }
}

impl FromIterator<User> for Vec<ResponseUser> {
    fn from_iter<I: IntoIterator<Item = User>>(iter: I) -> Self {
        let mut c = Self::new();

        for i in iter {
            c.push(ResponseUser {
                user_id: i.user_id,
                fullname: i.fullname,
                profile_photo: i.profile_photo,
                email: i.email,
                birth_place: i.birth_place,
                birth_date: i.birth_date,
                bio: i.bio,
                status: i.status,
                created_at: i.created_at,
            });
        }

        c
    }
}

impl Manipulable<Self> for User {
    fn create(new_data: Self, conn: &PgConnection) -> ThearningResult<Self> {
        let hashed = Self {
            password: hash(new_data.password, DEFAULT_COST).unwrap(),
            ..new_data
        };
        diesel::insert_into(users::table)
            .values(&hashed)
            .execute(conn)?;

        let res = users::table.find(hashed.user_id).get_result::<Self>(conn)?;

        Ok(res)
    }

    fn update(&self, update: Self, conn: &PgConnection) -> ThearningResult<Self> {
        diesel::update(users::table.filter(users::user_id.eq(&self.user_id)))
            .set((
                users::fullname.eq(&update.fullname),
                users::profile_photo.eq(&update.profile_photo),
                users::email.eq(&update.email),
                users::bio.eq(&update.bio),
                users::birth_place.eq(&update.birth_place),
                users::birth_date.eq(&update.birth_date),
            ))
            .execute(conn)?;

        let res = users::dsl::users
            .find(&self.user_id)
            .get_result::<Self>(conn)?;

        Ok(res)
    }

    fn delete(&self, conn: &PgConnection) -> ThearningResult<Self> {
        let res = diesel::delete(users::table.find(&self.user_id)).get_result::<Self>(conn)?;

        Ok(res)
    }

    fn get_all(conn: &PgConnection) -> ThearningResult<Vec<Self>> {
        let res = users::table.load::<Self>(conn)?;

        Ok(res)
    }
}

macro_rules! impl_classuser {
    ($u:ident, $d:ident) => {
        impl ClassUser for $u {
            fn create(
                uid: &String,
                class_id: &String,
                conn: &PgConnection,
            ) -> ThearningResult<Self> {
                let u = Self {
                    id: generate_random_id(),
                    user_id: uid.to_string(),
                    class_id: class_id.to_string(),
                    created_at: Local::now().naive_local(),
                };
                diesel::insert_into($d::table).values(&u).execute(conn)?;

                let res = $d::table
                    .filter($d::user_id.eq(u.user_id))
                    .get_result::<Self>(conn)?;

                Ok(res)
            }

            fn load_in_class(class_id: &String, conn: &PgConnection) -> ThearningResult<Vec<Self>> {
                Ok($d::table
                    .filter($d::class_id.eq(&class_id))
                    .load::<Self>(conn)?
                    .into_iter()
                    .collect::<Vec<Self>>())
            }

            fn find(uid: &String, conn: &PgConnection) -> ThearningResult<Vec<Self>> {
                Ok($d::table.filter($d::user_id.eq(uid)).load::<Self>(conn)?)
            }
        }
    };
}

impl_classuser! {Student, students}
impl_classuser! {Teacher, teachers}
impl_classuser! {Admin, admins}
