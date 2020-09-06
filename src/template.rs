use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "pages.html")]
pub struct PagesTemplate<'a> {
  pub title: &'a str,
  pub pages: &'a [PageItemTemplate],
}

pub struct PageItemTemplate {
  pub id: String,
  pub obsoleted: bool,
  pub url: String,
}

#[derive(Template)]
#[template(path = "page.html")]
pub struct PageTemplate<'a> {
  pub linked_by: &'a [PageItemTemplate],
  pub page_id: &'a str,
  pub page_url: &'a str,
  pub title: &'a str,
  pub title_url: &'a str,
  pub html: String,
  pub obsoleted_by: &'a [PageItemTemplate],
}

#[derive(Template)]
#[template(path = "titles.html")]
pub struct TitlesTemplate<'a> {
  pub show_all: bool,
  pub title: &'a str,
  pub titles: &'a [TitlesItemTemplate],
}

pub struct TitlesItemTemplate {
  pub obsoleted: bool,
  pub title: String,
  pub url: String,
}

#[derive(Template)]
#[template(path = "title.html")]
pub struct TitleTemplate<'a> {
  pub title: &'a str,
  pub title_url: &'a str,
  pub pages: &'a [PageItemTemplate],
}
