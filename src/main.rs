mod day1;
mod day11;
mod day12;
mod day4;
mod day6;
mod day7;
mod day8;

use rocket::get;
use rocket::http::Status;
use rocket::routes;

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .manage(day12::TimeKeeer::new())
        .mount("/", routes![index])
        .mount("/-1", routes![internal_server_error])
        .mount("/1", routes![day1::cube_the_bits])
        .mount(
            "/4",
            routes![day4::reindeer_cheer, day4::cursed_candy_eating_contest],
        )
        .mount("/6", routes![day6::never_count_on_elf])
        .mount(
            "/7",
            routes![
                day7::based_encoding_64th_edition,
                day7::secret_cookie_recipe
            ],
        )
        .mount(
            "/8",
            routes![day8::it_is_pikachu, day8::that_is_gonna_leave_dent],
        )
        .mount(
            "/11",
            routes![
                day11::served_on_a_silver_platter,
                day11::bull_mode_activated
            ],
        )
        .mount(
            "/12",
            routes![
                day12::set_time_persist,
                day12::get_time_persist,
                day12::unanimously_legendary_identifier,
                day12::let_santa_broil
            ],
        );

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
