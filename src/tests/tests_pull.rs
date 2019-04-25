#[cfg(test)]
mod test {
    use crate::rocket_launcher;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn pull() {
        let client = Client::new(rocket_launcher()).expect("valid rocket instance");
        let response = client.get("/pull").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
