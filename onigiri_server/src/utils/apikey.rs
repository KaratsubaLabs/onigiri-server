/// Generate a random apikey using stripped namespaced UUID
pub fn generate_apikey() -> String {
    use uuid::Uuid;

    // TODO read the namespace from env variable
    let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, b"karatsubalabs.com");
    uuid.simple()
        .encode_lower(&mut Uuid::encode_buffer())
        .to_owned()
}
