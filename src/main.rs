#![feature(proc_macro_hygiene,decl_macro)]
#![feature(in_band_lifetimes)]

#[macro_use]
extern crate rocket;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod controller;

fn main() {
    rocket::ignite()
        .mount("/",routes![controller::auth_controller::register])
        .launch();
}
