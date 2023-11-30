use rocket::get;
use rocket::http::Status;
use rocket::routes;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/-1/error")]
fn internal_server_error() -> Status {
    Status::InternalServerError
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![internal_server_error]);

    Ok(rocket.into())
}
