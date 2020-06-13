#![feature(proc_macro_hygiene,decl_macro)]
#![feature(in_band_lifetimes)]

#[macro_use]
extern crate rocket;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod controller;
mod routes;

fn main() {
    routes::build().launch();
}
