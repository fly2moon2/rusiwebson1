
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use std::collections::HashMap;

use rocket::response::content::Json;

use rocket_contrib::templates::Template;


#[get("/template")]
/* fn template() -> Template {
    //let context = context();
    let mut context =  HashMap::new();
    Template::render("index", &context);
    
 }  */
 fn tplate() -> Template {
    let context: HashMap<&str, &str> = [("name", "Jonathan"),("attemptTimes","first")]
        .iter().cloned().collect();
    Template::render("index", &context)
}

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
    rocket::ignite().mount("/",routes![index, jackson, wave, tplate, branch::world])
    .attach(Template::fairing())
    .launch();
}
