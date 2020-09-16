
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::sync::Mutex;
use std::collections::HashMap;

// ref:
// https://github.com/SergioBenitez/Rocket/blob/v0.4/examples/json/src/main.rs
//use rocket::response::content::Json;
use rocket_contrib::json::{Json,JsonValue};
use rocket::State;

// Note: rocket_contrib::templates::Template version 2018 not rocket_contrib::Template
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;


// The type to represent the ID of a message.
type MsgID = usize;

// We're going to store all of the messages here. No need for a DB.
type MessageMap = Mutex<HashMap<MsgID, String>>;

#[derive(Serialize, Deserialize)]
struct Message {
    id: Option<MsgID>,
    contents: String
}

// TODO: This example can be improved by using `route` with multiple HTTP verbs.
//$curl -X POST -d '{ "contents": "Hello, world!"}' -H 'Content-Type: application/json' http:/localhost:8000/message/1
#[post("/<id>", format = "json", data = "<message>")]
fn new(id: MsgID, message: Json<Message>, map: State<MessageMap>) -> JsonValue {
    let mut hashmap = map.lock().expect("map lock.");
    if hashmap.contains_key(&id) {
        json!({
            "status": "error",
            "reason": "ID exists. Try put."
        })
    } else {
        hashmap.insert(id, message.0.contents);
        json!({ "status": "ok" })
    }
}

// $curl -X PUT -d '{ "contents": "Harlo, wolf!"}' -H 'Content-Type: application/json' http:/localhost:8000/message/1
#[put("/<id>", format = "json", data = "<message>")]
fn update(id: MsgID, message: Json<Message>, map: State<MessageMap>) -> Option<JsonValue> {
    let mut hashmap = map.lock().unwrap();
    if hashmap.contains_key(&id) {
        hashmap.insert(id, message.0.contents);
        Some(json!({ "status": "ok" }))
    } else {
        None
    }
}

//$curl http://localhost:8000/message/1
#[get("/<id>", format = "json")]
fn get(id: MsgID, map: State<MessageMap>) -> Option<Json<Message>> {
    let hashmap = map.lock().unwrap();
    hashmap.get(&id).map(|contents| {
        Json(Message {
            id: Some(id),
            contents: contents.clone()
        })
    })
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

// ref:
// https://medium.com/@james_32022/rocket-frontend-templates-and-static-assets-5b6d04243a08
#[get("/template")]
 fn template() -> Template {
    let context: HashMap<&str, &str> = [("name", "Jonathan"),("attemptTimes","first")]
        .iter().cloned().collect();
    Template::render("index", &context)
}
/* fn template() -> Template {
    //let context = context();
    let mut context =  HashMap::new();
    Template::render("index", &context);
    
 }  */

 #[get("/jackson")]
fn jackson() -> Json<&'static str> {
// single-quote outer, double-quote inner to avoid the following error
    // SyntaxError: JSON.parse: expected property name or '}' 
   Json("{'status': 'success',
        'message': 'Hello API!'}") 

}

// sample module
mod branch {
    #[get("/branch")]
    pub fn world() -> &'static str {
        "branch, world!"
    }
}

#[get("/")]
fn index<'alifetime>() -> &'alifetime str {
// fn index() -> &'static str {
    "Hello, world!"
}

#[get("/opt?wave&<name>")]
fn wave(name: Option<String>) -> String {
    name.map(|name| format!("wave with name, {}!", name))
        .unwrap_or_else(|| "wave no name!".into())
}
/* 
#[derive(Serialize, Deserialize)]
struct Message {
   contents: String,
}

#[put("/<id>", data = "<msg>")]
fn update(db: &Db, id: Id, msg: Json<Message>) -> JsonValue {
    if db.contains_key(&id) {
        db.insert(id, &msg.contents);
        json!({ "status": "ok" })
    } else {
        json!({ "status": "error" })
    }
} */

fn main() {
    // println!("Hello, world!");
    //rocket::ignite().attach(Template::fairing());
    rocket::ignite()
    .mount("/static", StaticFiles::from("static"))
    .mount("/",routes![index, jackson, wave, template, branch::world])
    .attach(Template::fairing())
    .mount("/message", routes![new, update, get])
    .register(catchers![not_found])
    .manage(Mutex::new(HashMap::<MsgID, String>::new()))
    .launch();
}
