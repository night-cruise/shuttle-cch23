use rocket::get;

use std::path::PathBuf;

#[get("/<path..>")]
pub fn cube_the_bits(path: PathBuf) -> String {
    path
        .iter()
        .map(|item| item.to_str().unwrap().parse::<i32>().unwrap())
        .fold(0, |acc, item| acc ^ item)
        .pow(3)
        .to_string()
}