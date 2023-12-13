use rocket::form::Form;
use rocket::fs::{NamedFile, TempFile};
use rocket::tokio::io;
use rocket::FromForm;
use rocket::{get, post};

use image::GenericImageView;

use std::path::PathBuf;

#[get("/<path..>")]
pub async fn served_on_a_silver_platter(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(path).await.ok()
}

#[derive(FromForm)]
pub struct ImageUpload<'r> {
    image: TempFile<'r>,
}

#[post("/red_pixels", data = "<upload>")]
pub async fn bull_mode_activated(upload: Form<ImageUpload<'_>>) -> Option<String> {
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
