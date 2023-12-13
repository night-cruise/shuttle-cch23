use rocket::get;
use rocket::http::CookieJar;
use rocket::serde::json::{serde_json, Json};
use rocket::serde::{Deserialize, Serialize};

use base64::engine::general_purpose;
use base64::Engine;

use std::collections::{HashMap, HashSet};

#[get("/decode")]
pub fn based_encoding_64th_edition(cookies: &CookieJar<'_>) -> Option<String> {
    let recipe_encoded = cookies.get("recipe").map(|recipe| recipe.value())?;

    let recipe_decoded = general_purpose::STANDARD.decode(recipe_encoded).ok()?;

    let recipe = String::from_utf8(recipe_decoded).ok()?;

    Some(recipe)
}

#[derive(Deserialize, Debug)]
pub struct SecretCookieRecipeReqBody {
    recipe: HashMap<String, usize>,
    pantry: HashMap<String, usize>,
}

#[derive(Serialize)]
pub struct SecretCookieRecipeRespBody {
    cookies: usize,
    pantry: HashMap<String, usize>,
}

#[get("/bake")]
pub fn secret_cookie_recipe(cookies: &CookieJar<'_>) -> Option<Json<SecretCookieRecipeRespBody>> {
    let recipe_encoded = cookies.get("recipe").map(|recipe| recipe.value())?;

    let recipe_decoded = general_purpose::STANDARD.decode(recipe_encoded).ok()?;

    let SecretCookieRecipeReqBody { recipe, pantry } =
        serde_json::from_slice::<SecretCookieRecipeReqBody>(&recipe_decoded).ok()?;

    let ingredients = recipe
        .keys()
        .map(|key| key.as_str())
        .collect::<HashSet<&str>>()
        .intersection(&pantry.keys().map(|key| key.as_str()).collect())
        .copied()
        .collect::<Vec<&str>>();

    let mut cookie_counts = usize::MAX;
    for &ingredient in &ingredients {
        if *recipe.get(ingredient).unwrap() == 0 {
            continue;
        }
        cookie_counts = std::cmp::min(
            cookie_counts,
            pantry.get(ingredient).unwrap() / recipe.get(ingredient).unwrap(),
        );
    }
    if cookie_counts == usize::MAX {
        cookie_counts = 0;
    }

    if cookie_counts == 0 {
        return Some(Json(SecretCookieRecipeRespBody { cookies: 0, pantry }));
    }

    let mut remain_pantry = pantry.clone();
    for &ingredient in &ingredients {
        *remain_pantry.get_mut(ingredient).unwrap() -=
            cookie_counts * recipe.get(ingredient).unwrap();
    }

    Some(Json(SecretCookieRecipeRespBody {
        cookies: cookie_counts,
        pantry: remain_pantry,
    }))
}
