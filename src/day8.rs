use rocket::get;
use rocket::serde::Deserialize;

#[derive(Deserialize)]
struct ItIsPikachuRespBody {
    weight: usize,
}

#[get("/weight/<pokedex_number>")]
pub async fn it_is_pikachu(pokedex_number: i32) -> Option<String> {
    let pika = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{pokedex_number}"))
        .await
        .ok()?
        .json::<ItIsPikachuRespBody>()
        .await
        .ok()?;
    
    Some((pika.weight as f64 / 10.0).to_string())
}

#[get("/drop/<pokedex_number>")]
pub async fn that_is_gonna_leave_dent(pokedex_number: i32) -> Option<String> {
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