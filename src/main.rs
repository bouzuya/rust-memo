mod page_id;

use actix_web::{web, HttpResponse};
use askama::Template;
use clap;
use page_id::PageId;
use std::fs::{read_to_string, File};
use std::io::Write;

#[derive(Template)]
#[template(path = "pages.html")]
struct PagesTemplate<'a> {
    title: &'a str,
    pages: &'a [PageItemTemplate],
}

struct PageItemTemplate {
    id: String,
    url: String,
}

#[derive(Template)]
#[template(path = "page.html")]
struct PageTemplate<'a> {
    title: &'a str,
    html: String,
    obsoleted_by: &'a [PageItemTemplate],
}

#[derive(Template)]
#[template(path = "titles.html")]
struct TitlesTemplate<'a> {
    title: &'a str,
    titles: &'a [TitlesItemTemplate],
}

struct TitlesItemTemplate {
    title: String,
    url: String,
}

#[derive(Template)]
#[template(path = "title.html")]
struct TitleTemplate<'a> {
    title: &'a str,
    pages: &'a [PageItemTemplate],
}

fn to_file_name(page_id: &PageId) -> String {
    format!("{}.md", page_id.to_string())
}

fn to_page_url(page_id: &PageId) -> String {
    format!(
        "/pages/{}",
        percent_encoding::utf8_percent_encode(
            &page_id.to_string(),
            percent_encoding::NON_ALPHANUMERIC,
        )
    )
}

fn to_title_url(title: &str) -> String {
    format!(
        "/titles/{}",
        percent_encoding::utf8_percent_encode(title, percent_encoding::NON_ALPHANUMERIC)
    )
}

fn read_obsoletes(page_id: &PageId) -> Vec<PageId> {
    use regex::Regex;
    let re = Regex::new(r"^- \[(\d{4}\d{2}\d{2}T\d{2}\d{2}\d{2}Z)\]\(.*\)$").unwrap();
    let file_name = to_file_name(&page_id);
    let content = match std::fs::read_to_string(&file_name) {
        Ok(x) => x,
        Err(_) => return Vec::new(),
    };
    if let Some(index) = content.find("\n## Obsoletes") {
        let mut obsoletes = Vec::new();
        for line in content[index..].lines() {
            if let Some(caps) = re.captures(line) {
                let s = caps.get(1).unwrap().as_str();
                if let Some(page_id) = PageId::from_str(s) {
                    obsoletes.push(page_id);
                }
            }
        }
        obsoletes
    } else {
        return Vec::new();
    }
}

fn read_obsoleted_map(
) -> std::io::Result<std::collections::BTreeMap<PageId, std::collections::BTreeSet<PageId>>> {
    let mut map = std::collections::BTreeMap::new();
    let page_ids = list_ids()?;
    for &new_page_id in page_ids.iter() {
        let obsoletes = read_obsoletes(&new_page_id);
        for &old_page_id in obsoletes.iter() {
            map.entry(old_page_id)
                .or_insert(std::collections::BTreeSet::new())
                .insert(new_page_id);
        }
    }
    Ok(map)
}

fn read_title(page_id: &PageId) -> String {
    use std::io::prelude::*;
    let file = match std::fs::File::open(&to_file_name(page_id)) {
        Ok(file) => file,
        Err(_) => return String::new(),
    };
    let mut reader = std::io::BufReader::new(file);
    let mut buffer = String::new();
    match reader.read_line(&mut buffer) {
        Ok(_) => {}
        Err(_) => return String::new(),
    };
    if buffer.starts_with("# ") {
        return buffer[2..].trim().to_owned();
    } else {
        return String::new();
    }
}

fn read_title_map() -> std::io::Result<std::collections::BTreeMap<String, Vec<PageId>>> {
    let mut title_map = std::collections::BTreeMap::new();
    let page_ids = list_ids()?;
    for &page_id in page_ids.iter() {
        let title = read_title(&page_id);
        title_map.entry(title).or_insert(vec![]).push(page_id);
    }
    Ok(title_map)
}

fn create_new_file(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let page_id = PageId::new().expect("This application is out of date.");
    let file_name = to_file_name(&page_id);
    let mut file = File::create(&file_name)?;
    writeln!(file, "{}", content)?;
    file.flush()?;
    Ok(file_name)
}

fn edit_file(id_as_string: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let page_id = PageId::from_str(id_as_string).expect("invalid ID format");
    let old_file_name = to_file_name(&page_id);
    let mut content = read_to_string(&old_file_name)?;
    if let Some(index) = content.find("\n## Obsoletes") {
        content.truncate(index);
    }
    content.push_str(&format!(
        "\n## Obsoletes\n\n- [{}]({})",
        page_id.to_string(),
        to_page_url(&page_id)
    ));
    let new_file_name = create_new_file(&content)?;
    Ok((old_file_name, new_file_name))
}

fn list_ids() -> std::io::Result<Vec<PageId>> {
    let mut ids = vec![];
    for res in std::fs::read_dir(".")? {
        let dir_entry = res?;
        let file_type = dir_entry.file_type()?;
        if !file_type.is_file() {
            continue;
        }
        let path = dir_entry.path();
        let id_as_string = match path.file_stem().and_then(|os_str| os_str.to_str()) {
            Some(x) => x,
            None => continue,
        };
        if let Some(page_id) = PageId::from_str(id_as_string) {
            ids.push(page_id);
        }
    }
    ids.sort();
    Ok(ids)
}

async fn index() -> impl actix_web::Responder {
    HttpResponse::Found()
        .header(actix_web::http::header::LOCATION, format!("/pages"))
        .finish()
}

async fn pages() -> std::io::Result<HttpResponse> {
    let page_ids = list_ids()?;
    let pages = page_ids
        .iter()
        .map(|page_id| PageItemTemplate {
            id: page_id.to_string(),
            url: to_page_url(&page_id),
        })
        .collect::<Vec<PageItemTemplate>>();
    let template = PagesTemplate {
        title: "/pages",
        pages: &pages,
    };
    let html = template.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn page(params: web::Path<(String,)>) -> std::io::Result<HttpResponse> {
    let page_id = PageId::from_str(&params.0).ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "invalid page_id format",
    ))?;
    let obsoleted_map = read_obsoleted_map()?;
    let obsoleted_by = obsoleted_map
        .get(&page_id)
        .unwrap_or(&std::collections::BTreeSet::new())
        .iter()
        .map(|page_id| PageItemTemplate {
            id: page_id.to_string(),
            url: to_page_url(&page_id),
        })
        .collect::<Vec<PageItemTemplate>>();
    let page_file_name = to_file_name(&page_id);
    let md = std::fs::read_to_string(page_file_name)?;
    let parser = pulldown_cmark::Parser::new(&md);
    let mut markdown_html = String::new();
    pulldown_cmark::html::push_html(&mut markdown_html, parser);
    let template = PageTemplate {
        title: &to_page_url(&page_id),
        html: markdown_html,
        obsoleted_by: &obsoleted_by,
    };
    let html = template.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn titles() -> std::io::Result<HttpResponse> {
    let title_map = read_title_map()?;
    let titles = title_map
        .iter()
        .map(|(title, _)| TitlesItemTemplate {
            title: title.to_owned(),
            url: to_title_url(&title),
        })
        .collect::<Vec<TitlesItemTemplate>>();
    let template = TitlesTemplate {
        title: "/titles",
        titles: &titles,
    };
    let html = template.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn title(params: web::Path<(String,)>) -> std::io::Result<HttpResponse> {
    let title_map = read_title_map()?;
    let title = &params.0;
    if let Some(page_ids) = title_map.get(title) {
        let pages = page_ids
            .iter()
            .map(|page_id| PageItemTemplate {
                id: page_id.to_string(),
                url: to_page_url(&page_id),
            })
            .collect::<Vec<PageItemTemplate>>();
        let template = TitleTemplate {
            title: &to_title_url(title),
            pages: &pages,
        };
        let html = template.render().unwrap();
        Ok(HttpResponse::Ok().content_type("text/html").body(html))
    } else {
        Ok(HttpResponse::NotFound().body("Not Found"))
    }
}

#[actix_rt::main]
async fn run_server() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .route("/", web::get().to(index))
            .route("/pages", web::get().to(pages))
            .route("/pages/{id}", web::get().to(page))
            .route("/titles", web::get().to(titles))
            .route("/titles/{title}", web::get().to(title))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::App::new("memo")
        .subcommand(
            clap::SubCommand::with_name("edit")
                .about("Creates a new memo that obsoletes the specified memo")
                .arg(
                    clap::Arg::with_name("ID")
                        .help("the id of the memo to edit")
                        .required(true),
                ),
        )
        .subcommand(clap::SubCommand::with_name("new").about("Creates a new memo"))
        .subcommand(clap::SubCommand::with_name("server").about("Runs server"))
        .get_matches();
    match matches.subcommand() {
        ("new", _) => {
            let new = create_new_file("# ")?;
            println!("{}", new);
        }
        ("edit", Some(sub_matches)) => {
            let (old, new) = edit_file(sub_matches.value_of("ID").expect("ID required"))?;
            println!("{} -> {}", old, new);
        }
        ("server", _) => {
            run_server()?;
        }
        _ => {}
    }
    Ok(())
}
