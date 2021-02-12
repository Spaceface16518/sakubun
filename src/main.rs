#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use io::Read;
use multipart::server::Multipart;
use rocket::{http::{ContentType, Status}, request::Form, response::status::Custom, Config, Data};
use rocket_contrib::{serve::StaticFiles, templates::Template};
use std::env;
use std::collections::HashMap;
use std::io::{self, Cursor};
use std::fs;

mod sentences;

use sentences::*;

#[derive(FromForm)]
pub struct QuizSettings {
    min: usize,
    max: usize,
    known_kanji: String,
}

#[derive(FromForm)]
pub struct WaniKaniImport {
    number: usize,
    method: String,
}

#[get("/")]
fn get_index() -> Template {
    let mut context = HashMap::new();
    context.insert("page", "/");
    Template::render("index", context)
}

#[get("/known_kanji")]
fn get_known_kanji() -> Template {
    let mut context = HashMap::new();
    context.insert("page", "known_kanji");
    Template::render("known_kanji", context)
}

#[get("/quiz")]
fn get_quiz() -> Template {
    let mut context = HashMap::new();
    context.insert("page", "quiz");
    Template::render("quiz", context)
}

#[get("/custom_text")]
fn get_custom_text() -> Template {
    let mut context = HashMap::new();
    context.insert("page", "custom_text");
    Template::render("custom_text", context)
}

#[post("/sentences", data = "<quiz_settings>")]
fn post_sentences(quiz_settings: Form<QuizSettings>) -> String {
    get_sentences(quiz_settings).unwrap().iter().map(|x| x.join(";")).collect::<Vec<_>>().join("|")
}

#[post("/import_anki", data = "<data>")]
fn post_import_anki(cont_type: &ContentType, data: Data) -> Result<String, Custom<String>> {
    // Validate data
    if !cont_type.is_form_data() {
        return Err(Custom(
            Status::BadRequest,
            "Content-Type not multipart/form-data".into()
        ));
    }

    let (_, boundary) = cont_type.params().find(|&(k, _)| k == "boundary").ok_or_else(
            || Custom(
                Status::BadRequest,
                "`Content-Type: multipart/form-data` boundary param not provided".into()
            )
        )?;

    // Read data
    let mut include_unlearned = String::new();
    let mut buf = Vec::new();
    let mut form_data = Multipart::with_body(data.open(), boundary);
    form_data.read_entry().unwrap().unwrap().data.read_to_string(&mut include_unlearned).unwrap();
    form_data.read_entry().unwrap().unwrap().data.read_to_end(&mut buf).unwrap();
    // The maximum allowed file size is 4 MiB
    if buf.len() > 4194304 {
        return Err(Custom(Status::PayloadTooLarge, String::from("File too large")));
    }
    extract_kanji_from_anki_deck(Cursor::new(buf), include_unlearned == "true")
}

#[post("/import_wanikani", data = "<import_settings>")]
fn post_import_wanikani(import_settings: Form<WaniKaniImport>) -> Result<String, Custom<String>> {
    let wanikani_kanji = fs::read_to_string("wanikani.txt").unwrap();
    match import_settings.method.as_str() {
        "levels" => Ok(wanikani_kanji.split("\n").collect::<Vec<_>>()[..import_settings.number].join("")),
        "kanji" => Ok(wanikani_kanji.chars().filter(|c| c != &'\n').take(import_settings.number).collect()),
        _ => Err(Custom(Status::BadRequest, String::from("Method must be one of `levels` or `kanji`"))),
    }
}

fn configure() -> Config {
    let mut config = Config::active().expect("could not load configuration");
    // Configure Rocket to use the PORT env var or fall back to 8000
    let port = if let Ok(port_str) = env::var("PORT") {
        port_str.parse().expect("could not parse PORT")
    } else {
        8000
    };
    config.set_port(port);
    config
}

fn rocket() -> rocket::Rocket {
    rocket::custom(configure())
        .mount(
            "/",
            routes![
                get_index,
                get_known_kanji,
                get_quiz,
                get_custom_text,
                post_sentences,
                post_import_anki,
                post_import_wanikani,
            ],
        )
        .mount("/styles", StaticFiles::from("static/styles"))
        .mount("/scripts", StaticFiles::from("static/scripts"))
        .mount("/fonts", StaticFiles::from("static/fonts"))
        .mount("/dict", StaticFiles::from("static/dict"))
        .mount("/", StaticFiles::from("static/icons").rank(20))
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
