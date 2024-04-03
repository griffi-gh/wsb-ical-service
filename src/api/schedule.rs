use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
pub struct Semester {
  pub id: u32,
  pub name: String,
  pub name_eng: Option<String>,
}

#[derive(Deserialize)]
pub struct Subject {
  pub id: u32,
  pub name: String,
  pub name_eng: Option<String>,
}

#[derive(Deserialize)]
pub struct Form {
  pub id: u32,
  pub name: String,
  pub name_eng: Option<String>,
  #[serde(rename = "type")]
  pub type_id: u16,
  pub code: char,
}

#[derive(Deserialize)]
pub struct Lecturer {
  pub id: u32,
  pub name: String,
  pub email: String,
  pub absent: bool,
  //there are two more fields here, but I don't know how they work
  //(havent' reverse-engineered them yet due to lack of data in my schedule)
  //substitute: Option<...>,
  //duty: []
}

#[derive(Deserialize)]
pub struct Location {
  pub classroom: String,
  pub building_name: String,
  pub building_code: char,
  pub address: String,
  pub city: String,
  pub postal_code: String,
  pub lat: f64,
  pub lon: f64,
}

//TODO group

#[derive(Deserialize)]
pub struct Event {
  pub id: u32,
  pub school: String,
  pub semester: Semester,
  pub academic_year: u16,
  pub subject: Subject,
  pub form: Form,
  pub lecturers: Vec<Lecturer>,
  #[serde(deserialize_with = "deserialize_time")]
  pub begin_date: NaiveDateTime,
  #[serde(deserialize_with = "deserialize_time")]
  pub end_date: NaiveDateTime,
  pub lessons_hours: u8,
  pub locations: Vec<Location>,
  pub cancelled: bool,
  pub hours_to_date: u8,
  pub hours_total: u8,
  pub comments: Option<String>,
  pub comments_meeting: Option<String>,
  pub moved_from: Option<u32>,
  pub moved_to: Option<u32>,
  #[serde(deserialize_with = "deserialize_time_option")]
  pub moved_to_date: Option<NaiveDateTime>,
  //... moved_to, moved_from, etc
}

fn deserialize_time<'de, D: Deserializer<'de>>(
  deserializer: D
) -> Result<NaiveDateTime, D::Error> {
  let s = String::deserialize(deserializer)?;
  NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
    .map_err(serde::de::Error::custom)
}

fn deserialize_time_option<'de, D: Deserializer<'de>>(
  deserializer: D
) -> Result<Option<NaiveDateTime>, D::Error> {
  if let Some(str) = Option::<String>::deserialize(deserializer)? {
    let time = NaiveDateTime::parse_from_str(&str, "%Y-%m-%d %H:%M:%S")
      .map_err(serde::de::Error::custom)?;
    Ok(Some(time))
  } else {
    Ok(None)
  }
}
