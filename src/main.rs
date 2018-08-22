// Import rocket.rs to build a web server
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

// Import ./bLock.rs
mod block;

#[get("/")]
fn index() -> String {
	// Create a new Block
    let data = String::from("0x0");
	let block = block::Block::new(data);
    format!("It's easy to display custom data: \n {:?}", block)
}

fn main() {
	// Start Rocket Web Server
  rocket::ignite().mount("/", routes![index]).launch();
}
