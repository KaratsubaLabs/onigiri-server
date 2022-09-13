use rocket::Request;

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("not found")
}
