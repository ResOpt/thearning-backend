use crate::schema::announcements;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Serialize, Deserialize};
use crate::errors::ThearningResult;
use crate::traits::Manipulable;
use crate::utils::generate_random_id;

#[derive(Queryable, Serialize, Deserialize, Debug, Insertable, AsChangeset)]
#[table_name = "announcements"]
pub struct Announcement {
    pub announcement_id: String,
    pub announcement_name: Option<String>,
    pub class_id: Option<String>,
    pub posted_date: NaiveDate,
    pub body: Option<String>,
    pub draft: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct FillableAnnouncement {
    pub announcement_id: String,
    pub announcement_name: Option<String>,
    pub class_id: Option<String>,
    pub body: Option<String>,
}

impl Announcement {
    pub fn load_in_class(conn: &PgConnection, class_id: impl AsRef<str>) -> ThearningResult<Vec<Announcement>> {
        announcements::table
            .filter(announcements::class_id.eq(class_id.as_ref()))
            .load::<Announcement>(conn)
            .map_err(|e| e.into())
    }

    pub fn find_announcement(conn: &PgConnection, announcement_id: &str) -> ThearningResult<Announcement> {
        announcements::table
            .filter(announcements::announcement_id.eq(announcement_id))
            .first::<Announcement>(conn)
            .map_err(|e| e.into())
    }

    pub fn draft(&self, conn: &PgConnection) -> ThearningResult<Self> {
        diesel::insert_into(announcements::table)
            .values(&*self)
            .get_result::<Announcement>(conn)
            .map_err(|e| e.into())
    }
}

impl Default for Announcement {
    fn default() -> Announcement {
        Announcement {
            announcement_id: format!("{}", generate_random_id()),
            announcement_name: None,
            class_id: None,
            posted_date: Local::today().naive_local(),
            draft: true,
            body: None,
            created_at: Local::now().naive_local(),
        }
    }
}

impl Manipulable<FillableAnnouncement> for Announcement {
    fn create(new_data: FillableAnnouncement, conn: &PgConnection) -> ThearningResult<Self> {
        unimplemented!()
    }

    fn update(&self, update: FillableAnnouncement, conn: &PgConnection) -> ThearningResult<Self> {
        let announcement = Announcement {
            announcement_id: self.announcement_id.clone(),
            announcement_name: update.announcement_name,
            class_id: update.class_id,
            posted_date: self.posted_date,
            draft: false,
            body: update.body,
            created_at: self.created_at,
        };
        Ok(diesel::update(announcements::table.find(self.announcement_id.clone()))
            .set(&announcement)
            .get_result::<Announcement>(conn)?)
    }

    fn delete(&self, conn: &PgConnection) -> ThearningResult<Self> {
        Ok(diesel::delete(announcements::table.find(self.announcement_id.clone()))
            .get_result::<Announcement>(conn)?)
    }

    fn get_all(conn: &PgConnection) -> ThearningResult<Vec<Self>> {
        todo!()
    }
}


