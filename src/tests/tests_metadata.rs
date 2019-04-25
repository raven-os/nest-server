#[cfg(test)]
mod test {
    use crate::rocket_launcher;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn metadata_ok_1() {
        let client = Client::new(rocket_launcher()).expect("valid rocket instance");
        let mut response = client.get("/p/kernel/linux/4.15.3/metadata").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("{\"metadata\":{\"name\":\"linux\",\"category\":\"kernel\",\"version\":\"4.15.3\",\"description\":\"An awesome kernel\",\"tags\":\"linux, kernel, unix, basic\",\"created_at\":\"1992-04-06T20:01:01Z\"},\"dependencies\":{}}".into()));
    }

    #[test]
    fn metadata_ok_2() {
        let client = Client::new(rocket_launcher()).expect("valid rocket instance");
        let mut response = client.get("/p/sys-lib/libc/2.27.0/metadata").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("{\"metadata\":{\"name\":\"libc\",\"category\":\"sys-lib\",\"version\":\"2.27.0\",\"description\":\"The lib!\",\"tags\":\"libc, linux, unix, c\",\"created_at\":\"1993-04-06T20:01:01Z\"},\"dependencies\":{\"stable::sys-lib/linux-headers\":\"= 4.15.3\"}}".into()));
    }

    #[test]
    fn metadata_ok_3() {
        let client = Client::new(rocket_launcher()).expect("valid rocket instance");
        let mut response = client
            .get("/p/virtual/essentials/0.0.1/metadata")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("{\"metadata\":{\"name\":\"essentials\",\"category\":\"virtual\",\"version\":\"0.0.1\",\"description\":\"Essentials tools\",\"tags\":\"essential, tool, virtual, test\",\"created_at\":\"1971-04-06T20:01:01Z\"},\"dependencies\":{\"stable::sys-lib/libc\":\">= 2.27.0\"}}".into()));
    }

    #[test]
    fn metadata_404() {
        let client = Client::new(rocket_launcher()).expect("valid rocket instance");
        let mut response = client.get("/p/foo/bar/1.0.1/metadata").dispatch();
        assert_eq!(response.status(), Status::NotFound);
        assert_eq!(
            response.body_string(),
            Some("{\"reason\":\"Resource was not found.\",\"status\":\"error\"}".into())
        );
    }
}
