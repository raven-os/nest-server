#[cfg(test)]
mod test {
    use crate::rocket_launcher;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn download_ok() {
        let client = Client::new(rocket_launcher()).expect("valid rocket instance");
        let response = client
            .get("/p/virtual/essentials/0.0.1/download")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn download_404() {
        let client = Client::new(rocket_launcher()).expect("valid rocket instance");
        let mut response = client.get("/p/foo/bar/1.0.1/download").dispatch();
        assert_eq!(response.status(), Status::NotFound);
        assert_eq!(
            response.body_string(),
            Some("{\"reason\":\"Resource was not found.\",\"status\":\"error\"}".into())
        );
    }
}
