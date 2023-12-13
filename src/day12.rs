use rocket::serde::json::{json, Json, Value};
use rocket::serde::Serialize;
use rocket::State;
use rocket::{get, post};

use ulid::Ulid;
use uuid::Uuid;

use chrono::{Datelike, NaiveDateTime, Utc};

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Mutex;
use std::time::SystemTime;

pub struct TimeKeeer {
    inner: Mutex<HashMap<String, SystemTime>>,
}

impl TimeKeeer {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
        }
    }
}

#[post("/save/<s>")]
pub fn set_time_persist(s: &str, time_keeper: &State<TimeKeeer>) {
    time_keeper
        .inner
        .lock()
        .unwrap()
        .insert(s.to_owned(), SystemTime::now());
}

#[get("/load/<s>")]
pub fn get_time_persist(s: &str, time_keeper: &State<TimeKeeer>) -> Option<String> {
    time_keeper
        .inner
        .lock()
        .unwrap()
        .get(s)
        .map(|now| now.elapsed().map(|d| d.as_secs()).unwrap_or(0).to_string())
}

#[post("/ulids", data = "<ulids>")]
pub fn unanimously_legendary_identifier(ulids: Json<Vec<&str>>) -> Value {
    let res = ulids
        .iter()
        .rev()
        .map(|&ulid| {
            Uuid::from_bytes(Ulid::from_str(ulid).unwrap().to_bytes())
                .hyphenated()
                .to_string()
        })
        .collect::<Vec<String>>();

    json!(res)
}

#[derive(Serialize)]
pub struct LetSantaBroilRespBody {
    #[serde(rename = "christmas eve")]
    christmas_eve: i32,
    weekday: i32,
    #[serde(rename = "in the future")]
    in_the_future: i32,
    #[serde(rename = "LSB is 1")]
    lsb_is_1: i32,
}

#[post("/ulids/<weekday>", data = "<ulids>")]
pub fn let_santa_broil(weekday: i32, ulids: Json<Vec<&str>>) -> Json<LetSantaBroilRespBody> {
    let mut res = LetSantaBroilRespBody {
        christmas_eve: 0,
        weekday: 0,
        in_the_future: 0,
        lsb_is_1: 0,
    };
    for &ulid in ulids.iter() {
        let ulid = Ulid::from_str(ulid).unwrap();
        let timestamp = ulid.timestamp_ms();

        let datetime = NaiveDateTime::from_timestamp_opt(
            (timestamp / 1000) as i64,
            (timestamp % 1000) as u32 * 1_000_000,
        )
        .unwrap();
        if datetime.month() == 12 && datetime.day() == 24 {
            res.christmas_eve += 1;
        }
        if datetime.weekday().number_from_monday() - 1 == weekday as u32 {
            res.weekday += 1;
        }
        if timestamp > Utc::now().timestamp_millis() as u64 {
            res.in_the_future += 1;
        }
        if ulid.to_bytes()[15] & 1 == 1 {
            res.lsb_is_1 += 1;
        }
    }
    Json(res)
}
