use entity::PageTitle;

pub fn pages_url() -> String {
    "/pages".to_string()
}

pub fn title_url(title: &PageTitle) -> String {
    format!(
        "{}/{}",
        titles_url(),
        percent_encoding::utf8_percent_encode(title.as_str(), percent_encoding::NON_ALPHANUMERIC)
    )
}

pub fn titles_url() -> String {
    "/titles".to_string()
}
