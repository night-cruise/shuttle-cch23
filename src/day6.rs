use rocket::post;
use rocket::serde::json::{Value, json};

use regex::Regex;

#[post("/", data = "<text>")]
pub fn never_count_on_elf(text: &str) -> Value {  
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