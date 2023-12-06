#![allow(unused)]

use rocket::http::Status;
use rocket::routes;
use rocket::serde::json::{json, Json, Value, self};
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post, serde};

use std::path::PathBuf;

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/-1", routes![internal_server_error])
        .mount("/1", routes![cube_the_bits])
        .mount("/4", routes![reindeer_cheer, cursed_candy_eating_contest])
        .mount("/6", routes![never_count_on_elf]);
    Ok(rocket.into())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/error")]
fn internal_server_error() -> Status {
    Status::InternalServerError
}

#[get("/<path..>")]
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

#[post("/strength", data = "<reindeers>")]
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

#[post("/contest", data = "<reindeers>")]
fn cursed_candy_eating_contest(reindeers: Json<Vec<Reindeer>>) -> Json<ReindeerSummary> {
    let findest = |f: &dyn Fn(usize, usize) -> usize| {
        reindeers
            .iter()
            .enumerate()
            .fold(0, |acc, (i, _)| f(acc, i))
    };

    let fastest = findest(&|acc, i| if reindeers[acc].speed < reindeers[i].speed { i } else { acc });
    let fastest_value = format!("Speeding past the finish line with a strength of {} is {}", reindeers[fastest].strength, reindeers[fastest].name);

    let tallest = findest(&|acc, i| if reindeers[acc].height < reindeers[i].height { i } else { acc });
    let tallest_value = format!("{} is standing tall with his {} cm wide antlers", reindeers[tallest].name, reindeers[tallest].antler_width);

    let magician = findest(&|acc, i| if reindeers[acc].snow_magic_power < reindeers[i].snow_magic_power { i } else { acc });
    let magician_value = format!("{} could blast you away with a snow magic power of {}", reindeers[magician].name, reindeers[magician].snow_magic_power);

    let consumer = findest(&|acc, i| if reindeers[acc].c_an_d13s_3_a_te_n_yes_t3rd_ay < reindeers[i].c_an_d13s_3_a_te_n_yes_t3rd_ay { i } else { acc });
    let consumer_value = format!("{} ate lots of candies, but also some {}", reindeers[consumer].name, reindeers[consumer].favorite_food);

    Json(ReindeerSummary {
        fastest: fastest_value,
        tallest: tallest_value,
        magician: magician_value,
        consumer: consumer_value
    })
}

#[post("/", data = "<text>")]
fn never_count_on_elf(text: &str) -> Value {   
    let elf_counts = text.matches("elf").count();
    let elf_on_shelf_counts = text.matches("elf on a shelf").count();
    let mut shelf_without_elf_on_counts = text.matches("shelf").count() - text.matches("elf on a shelf").count();

    json!({
        "elf": elf_counts,
        "elf on a shelf": elf_on_shelf_counts,
        "shelf with no elf on it": shelf_without_elf_on_counts
    })
}