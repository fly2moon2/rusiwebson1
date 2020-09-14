
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

// sample module
mod other {
    #[get("/other")]
    pub fn worlds() -> &'static str {
        "other world!"
    }
}