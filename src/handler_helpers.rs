pub fn is_all(req: &actix_web::HttpRequest) -> bool {
    use std::str::FromStr;
    match url::Url::from_str(&format!("http://example.com{}", req.uri().to_string())) {
        Err(_) => false,
        Ok(url) => {
            let map = url
                .query_pairs()
                .into_owned()
                .collect::<std::collections::HashMap<String, String>>();
            map.get("all") == Some(&"true".to_owned())
        }
    }
}
