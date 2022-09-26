use rocket::{Catcher, Request};

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("not found")
}

#[catch(500)]
fn internal(req: &Request) -> String {
    format!("internal server error")
}

pub fn catchers() -> Vec<Catcher> {
    catchers![not_found, internal]
}
