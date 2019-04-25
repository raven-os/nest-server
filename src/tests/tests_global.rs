#[cfg(test)]
mod test {
    use crate::rocket_launcher;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn error_404() {
        let client = Client::new(rocket_launcher()).expect("valid rocket instance");
        let mut response = client.get("/foo/bar").dispatch();
        assert_eq!(response.status(), Status::NotFound);
        assert_eq!(
            response.body_string(),
            Some("{\"reason\":\"Resource was not found.\",\"status\":\"error\"}".into())
        );
    }
}
