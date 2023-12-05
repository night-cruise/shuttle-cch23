#![allow(unused)]

use rocket::http::Status;
use rocket::routes;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post, serde};

use std::path::PathBuf;

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![internal_server_error, cube_the_bits])
        .mount("/", routes![reindeer_cheer, cursed_candy_eating_contest]);
    Ok(rocket.into())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/-1/error")]
fn internal_server_error() -> Status {
    Status::InternalServerError
}

#[get("/1/<path..>")]
fn cube_the_bits(path: PathBuf) -> String {
    path
        .iter()
        .map(|item| item.to_str().unwrap().parse::<i32>().unwrap())
        .fold(0, |acc, item| acc ^ item)
        .pow(3)
        .to_string()
}

#[derive(Deserialize)]
struct Reindeer {
    name: String,
    strength: i32,
    #[serde(default)]
    speed: f32,
    #[serde(default)]
    height: i32,
    #[serde(default)]
    antler_width: i32,
    #[serde(default)]
    snow_magic_power: i32,
    #[serde(default)]
    favorite_food: String,
    #[serde(default)]
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    c_an_d13s_3_a_te_n_yes_t3rd_ay: i32
}

type Reindeers = Vec<Reindeer>;

#[post("/4/strength", data = "<reindeers>")]
fn reindeer_cheer(reindeers: Json<Reindeers>) -> String {
    reindeers
        .iter()
        .fold(0, |acc, item| acc + item.strength)
        .to_string()
}

#[derive(Serialize)]
struct ReindeerSummary {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String
}

#[post("/4/contest", data = "<reindeers>")]
fn cursed_candy_eating_contest(reindeers: Json<Vec<Reindeer>>) -> Json<ReindeerSummary> {
    let fastest = reindeers
        .iter()
        .enumerate()
        .fold(0, |acc, (i, item)| { if reindeers[acc].speed < item.speed { i } else { acc }});
    let fastest_value = format!("Speeding past the finish line with a strength of {} is {}", reindeers[fastest].strength, reindeers[fastest].name);

    let tallest = reindeers
        .iter()
        .enumerate()
        .fold(0, |acc, (i, item)| { if reindeers[acc].height < item.height { i } else { acc }});
    let tallest_value = format!("{} is standing tall with his {} cm wide antlers", reindeers[tallest].name, reindeers[tallest].antler_width);

    let magician = reindeers
        .iter()
        .enumerate()
        .fold(0, |acc, (i, item)| { if reindeers[acc].snow_magic_power < item.snow_magic_power { i } else { acc }});
    let magician_value = format!("{} could blast you away with a snow magic power of {}", reindeers[magician].name, reindeers[magician].snow_magic_power);

    let consumer = reindeers
        .iter()
        .enumerate()
        .fold(0, |acc, (i, item)| { if reindeers[acc].c_an_d13s_3_a_te_n_yes_t3rd_ay < item.c_an_d13s_3_a_te_n_yes_t3rd_ay { i } else { acc }});
    let consumer_value = format!("{} ate lots of candies, but also some {}", reindeers[consumer].name, reindeers[consumer].favorite_food);

    Json(ReindeerSummary {
        fastest: fastest_value,
        tallest: tallest_value,
        magician: magician_value,
        consumer: consumer_value
    })
}
