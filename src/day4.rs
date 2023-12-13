use rocket::post;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;


#[derive(Deserialize)]
pub struct Reindeer {
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
pub fn reindeer_cheer(reindeers: Json<Reindeers>) -> String {
    reindeers
        .iter()
        .fold(0, |acc, item| acc + item.strength)
        .to_string()
}

#[derive(Serialize)]
pub struct ReindeerSummary {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String
}

#[post("/contest", data = "<reindeers>")]
pub fn cursed_candy_eating_contest(reindeers: Json<Vec<Reindeer>>) -> Json<ReindeerSummary> {
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