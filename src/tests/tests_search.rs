#[cfg(test)]
mod test {
    use crate::rocket_launcher;
    use rocket::http::ContentType;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn search() {
        let client = Client::new(rocket_launcher()).expect("valid rocket instance");
        let mut response = client.post("/search").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("[{\"metadata\":{\"name\":\"linux\",\"category\":\"kernel\",\"version\":\"4.15.3\",\"description\":\"An awesome kernel\",\"tags\":\"linux, kernel, unix, basic\",\"created_at\":\"1992-04-06T20:01:01Z\"},\"dependencies\":{}},{\"metadata\":{\"name\":\"libc\",\"category\":\"sys-lib\",\"version\":\"2.27.0\",\"description\":\"The lib!\",\"tags\":\"libc, linux, unix, c\",\"created_at\":\"1993-04-06T20:01:01Z\"},\"dependencies\":{\"stable::sys-lib/linux-headers\":\"= 4.15.3\"}},{\"metadata\":{\"name\":\"essentials\",\"category\":\"virtual\",\"version\":\"0.0.1\",\"description\":\"Essentials tools\",\"tags\":\"essential, tool, virtual, test\",\"created_at\":\"1971-04-06T20:01:01Z\"},\"dependencies\":{\"stable::sys-lib/libc\":\">= 2.27.0\"}}]".into()));
    }

    #[test]
    fn search_filter_name_one_result() {
        let client = Client::new(rocket_launcher()).expect("valid rocket instance");
        let mut response = client
            .post("/search")
            .header(ContentType::Form)
            .body("description=Essentials")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("[{\"metadata\":{\"name\":\"essentials\",\"category\":\"virtual\",\"version\":\"0.0.1\",\"description\":\"Essentials tools\",\"tags\":\"essential, tool, virtual, test\",\"created_at\":\"1971-04-06T20:01:01Z\"},\"dependencies\":{\"stable::sys-lib/libc\":\">= 2.27.0\"}}]".into()));
    }

    #[test]
    fn search_filter_name_multiple_result() {
        let client = Client::new(rocket_launcher()).expect("valid rocket instance");
        let mut response = client
            .post("/search")
            .header(ContentType::Form)
            .body("name=li")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("[{\"metadata\":{\"name\":\"linux\",\"category\":\"kernel\",\"version\":\"4.15.3\",\"description\":\"An awesome kernel\",\"tags\":\"linux, kernel, unix, basic\",\"created_at\":\"1992-04-06T20:01:01Z\"},\"dependencies\":{}},{\"metadata\":{\"name\":\"libc\",\"category\":\"sys-lib\",\"version\":\"2.27.0\",\"description\":\"The lib!\",\"tags\":\"libc, linux, unix, c\",\"created_at\":\"1993-04-06T20:01:01Z\"},\"dependencies\":{\"stable::sys-lib/linux-headers\":\"= 4.15.3\"}}]".into()));
    }

    #[test]
    fn search_multiple_filter() {
        let client = Client::new(rocket_launcher()).expect("valid rocket instance");
        let mut response = client
            .post("/search")
            .header(ContentType::Form)
            .body("name=li&category=kernel")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("[{\"metadata\":{\"name\":\"linux\",\"category\":\"kernel\",\"version\":\"4.15.3\",\"description\":\"An awesome kernel\",\"tags\":\"linux, kernel, unix, basic\",\"created_at\":\"1992-04-06T20:01:01Z\"},\"dependencies\":{}}]".into()));
    }

    #[test]
    fn search_bad_filter_key() {
        let client = Client::new(rocket_launcher()).expect("valid rocket instance");
        let mut response = client
            .post("/search")
            .header(ContentType::Form)
            .body("foo=linux")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("[{\"metadata\":{\"name\":\"linux\",\"category\":\"kernel\",\"version\":\"4.15.3\",\"description\":\"An awesome kernel\",\"tags\":\"linux, kernel, unix, basic\",\"created_at\":\"1992-04-06T20:01:01Z\"},\"dependencies\":{}},{\"metadata\":{\"name\":\"libc\",\"category\":\"sys-lib\",\"version\":\"2.27.0\",\"description\":\"The lib!\",\"tags\":\"libc, linux, unix, c\",\"created_at\":\"1993-04-06T20:01:01Z\"},\"dependencies\":{\"stable::sys-lib/linux-headers\":\"= 4.15.3\"}},{\"metadata\":{\"name\":\"essentials\",\"category\":\"virtual\",\"version\":\"0.0.1\",\"description\":\"Essentials tools\",\"tags\":\"essential, tool, virtual, test\",\"created_at\":\"1971-04-06T20:01:01Z\"},\"dependencies\":{\"stable::sys-lib/libc\":\">= 2.27.0\"}}]".into()));
    }

    #[test]
    fn search_bad_filter_value() {
        let client = Client::new(rocket_launcher()).expect("valid rocket instance");
        let mut response = client
            .post("/search")
            .header(ContentType::Form)
            .body("name=foobar")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("[]".into()));
    }

    #[test]
    fn search_order_by_date_desc() {
        let client = Client::new(rocket_launcher()).expect("valid rocket instance");
        let mut response = client
            .post("/search")
            .header(ContentType::Form)
            .body("sort_by=date&order_by=desc")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("[{\"metadata\":{\"name\":\"essentials\",\"category\":\"virtual\",\"version\":\"0.0.1\",\"description\":\"Essentials tools\",\"tags\":\"essential, tool, virtual, test\",\"created_at\":\"1971-04-06T20:01:01Z\"},\"dependencies\":{\"stable::sys-lib/libc\":\">= 2.27.0\"}},{\"metadata\":{\"name\":\"libc\",\"category\":\"sys-lib\",\"version\":\"2.27.0\",\"description\":\"The lib!\",\"tags\":\"libc, linux, unix, c\",\"created_at\":\"1993-04-06T20:01:01Z\"},\"dependencies\":{\"stable::sys-lib/linux-headers\":\"= 4.15.3\"}},{\"metadata\":{\"name\":\"linux\",\"category\":\"kernel\",\"version\":\"4.15.3\",\"description\":\"An awesome kernel\",\"tags\":\"linux, kernel, unix, basic\",\"created_at\":\"1992-04-06T20:01:01Z\"},\"dependencies\":{}}]".into()));
    }
}
