#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;


use rocket_contrib::json::{Json};


#[derive(Serialize, Deserialize)]
struct Message {
    id: Option<u8>,
    contents: String
}

//  curl -H "Content-Type: application/json" -d '{"contents":"Fifty-three"}' -X POST http://localhost:8000/message
#[post("/", format = "json", data = "<message>")]
fn new( message: Json<Message>) -> Json<Message> {
    let mut response = message;
    response.contents = format!("You said: '{}'", response.contents);
    response.id = Some(response.id.unwrap_or(1)*2);
    return response;
}



#[catch(404)]
fn not_found() -> String {
    format!("")
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/message", routes![new])
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
