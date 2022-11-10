#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    return format!("Hello, {} years old named {}!", age, name);
}

fn main() {
    rocket::ignite()
        .mount("/hello", routes![hello])
        .launch();
}
