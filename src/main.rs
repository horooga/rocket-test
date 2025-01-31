mod questions;

use questions::*;
use rocket::{
    fs::FileServer,
    response::{Flash, Redirect},
    request::FlashMessage,
    *,
};
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn root() -> Template {
    Template::render("start", context![])
}

#[get("/result")]
fn result(flash: Option<FlashMessage>) -> Template {
    if let Some(ref x) = flash {
        let val = x.message().to_string().parse::<u8>().unwrap();
        Template::render("result", context![win:val])
    } else {
        Template::render("result", context![win:0])
    }
}

#[get("/question/<n>")]
fn question(n: u8) -> Template {
    let variants = VARIANTS[n as usize];
    Template::render("question", context![dn: n + 1, n: n, question: QUESTIONS[n as usize], variant0: variants[0], variant1: variants[1], variant2: variants[2]])
}

#[post("/question/<n>/<v>")]
fn next_question(n: u8, v: u8) -> Flash<Redirect> {
    if ANSWERS[n as usize] == v {
        if n != 1 {
            Flash::success(Redirect::to(uri!(question(n + 1))), "")
        } else {
            Flash::success(Redirect::to(uri!(result())), "1")
        }
    } else {
        Flash::success(Redirect::to(uri!(result())), "0")
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("."))
        .mount("/", routes![root, result, question, next_question])
        .attach(Template::fairing())
}
