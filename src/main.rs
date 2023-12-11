#![allow(unused)]


use rocket::tokio::io;
use rocket::form::Form;
use rocket::{get, post, serde};
use rocket::{routes, FromForm};
use rocket::fs::{NamedFile, TempFile};
use rocket::http::{Status, CookieJar};
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::{json, Json, Value, serde_json};

use base64::Engine;
use base64::engine::general_purpose;

use image::GenericImageView;

use regex::Regex;

use std::error;
use std::path::PathBuf;
use std::collections::{HashMap, HashSet};

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/-1", routes![internal_server_error])
        .mount("/1", routes![cube_the_bits])
        .mount("/4", routes![reindeer_cheer, cursed_candy_eating_contest])
        .mount("/6", routes![never_count_on_elf])
        .mount("/7", routes![based_encoding_64th_edition, secret_cookie_recipe])
        .mount("/8", routes![it_is_pikachu, that_is_gonna_leave_dent])
        .mount("/11", routes![served_on_a_silver_platter, bull_mode_activated]);

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
    let find = |pattern| {
        let mut start = 0;
        let mut counts = 0usize;
        while let Some(mat) = Regex::new(pattern).unwrap().find_at(text, start) {
            counts += 1;
            start = mat.start() + 1;
        }
        counts
    }; 
    let elf_counts = Regex::new("elf").unwrap().find_iter(text).count();
    let elf_on_shelf_counts = find("elf on a shelf");
    let shelf_without_elf_on_counts = Regex::new("shelf").unwrap().find_iter(text).count() - find("elf on a shelf");

    json!({
        "elf": elf_counts,
        "elf on a shelf": elf_on_shelf_counts,
        "shelf with no elf on it": shelf_without_elf_on_counts
    })
}

#[get("/decode")]
fn based_encoding_64th_edition(cookies: &CookieJar<'_>) -> Option<String> {
    let recipe_encoded = cookies.get("recipe").map(|recipe| recipe.value())?;
    
    let recipe_decoded = general_purpose::STANDARD.decode(recipe_encoded).ok()?;

    let recipe = String::from_utf8(recipe_decoded).ok()?;

    Some(recipe)
}

#[derive(Deserialize, Debug)]
struct SecretCookieRecipeReqBody {
    recipe: HashMap<String, usize>,
    pantry: HashMap<String, usize>,
}

#[derive(Serialize)] 
struct SecretCookieRecipeRespBody {
    cookies: usize,
    pantry: HashMap<String, usize>,
}

#[get("/bake")]
fn secret_cookie_recipe(cookies: &CookieJar<'_>) -> Option<Json<SecretCookieRecipeRespBody>> {
    let recipe_encoded = cookies.get("recipe").map(|recipe| recipe.value())?;
    
    let recipe_decoded = general_purpose::STANDARD.decode(recipe_encoded).ok()?;

    let SecretCookieRecipeReqBody { recipe, pantry } = serde_json::from_slice::<SecretCookieRecipeReqBody>(&recipe_decoded).ok()?;

    let ingredients = recipe
        .keys()
        .map(|key| key.as_str())
        .collect::<HashSet<&str>>()
        .intersection(&pantry.keys().map(|key| key.as_str()).collect())
        .map(|&key| key)
        .collect::<Vec<&str>>();

    let mut cookie_counts = usize::MAX;
    for &ingredient in &ingredients {
        if *recipe.get(ingredient).unwrap() == 0 {
            continue;
        }
        cookie_counts = std::cmp::min(cookie_counts, pantry.get(ingredient).unwrap() / recipe.get(ingredient).unwrap());
    }
    if cookie_counts == usize::MAX {
        cookie_counts = 0;
    }

    if cookie_counts == 0 {
        return Some(Json(SecretCookieRecipeRespBody{ cookies: 0, pantry }));
    }

    let mut remain_pantry = pantry.clone();
    for &ingredient in &ingredients {
        *remain_pantry.get_mut(ingredient).unwrap() -= cookie_counts * recipe.get(ingredient).unwrap();
    }

    Some(Json(SecretCookieRecipeRespBody{ cookies: cookie_counts, pantry: remain_pantry }))
}

#[derive(Deserialize)]
struct ItIsPikachuRespBody {
    weight: i32,
}

#[get("/weight/<pokedex_number>")]
async fn it_is_pikachu(pokedex_number: i32) -> Option<String> {
    let pika = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{pokedex_number}"))
        .await
        .ok()?
        .json::<ItIsPikachuRespBody>()
        .await
        .ok()?;
    
    Some((pika.weight as f64 / 10.0).to_string())
}

#[get("/drop/<pokedex_number>")]
async fn that_is_gonna_leave_dent(pokedex_number: i32) -> Option<String> {
    let pika = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{pokedex_number}"))
        .await
        .ok()?
        .json::<ItIsPikachuRespBody>()
        .await
        .ok()?;

    let v = ((2.0 * 9.825 * 10.0) as f64).sqrt();
    let p = (pika.weight as f64 / 10.0) * v;

    Some(p.to_string())
}

#[get("/<path..>")]
async fn served_on_a_silver_platter(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(path).await.ok()
}

#[derive(FromForm)]
struct ImageUpload<'r> {
    image: TempFile<'r>,
}

#[post("/red_pixels", data = "<upload>")]
async fn bull_mode_activated(upload: Form<ImageUpload<'_>>) -> Option<String> {
    let mut stream = upload.image.open().await.ok()?;

    let mut image = Vec::new();
    io::copy(&mut stream, &mut image).await.ok()?;

    let img = image::load_from_memory(&image).ok()?;
    let (width, height) = img.dimensions();

    let mut magical_red = 0;
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            if (pixel[1] as u16 + pixel[2] as u16) < pixel[0] as u16 {
                magical_red += 1;
            }
        }
    }

    Some(magical_red.to_string())
}