mod page_id;

use actix_web::{web, HttpResponse};
use clap;
use page_id::PageId;
use std::fs::{read_to_string, File};
use std::io::Write;

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
    let s = format!(
        "<html><head><title>/pages</title><body><h1>/pages</h1><ul>{}</ul></body></html>",
        page_ids
            .iter()
            .map(|page_id| {
                format!(
                    "<li><a href=\"{}\">{}</a></li>",
                    to_page_url(&page_id),
                    page_id.to_string()
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    );
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

async fn page(params: web::Path<(String,)>) -> std::io::Result<HttpResponse> {
    let page_id = PageId::from_str(&params.0).ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "invalid page_id format",
    ))?;
    let page_file_name = to_file_name(&page_id);
    let md = std::fs::read_to_string(page_file_name)?;
    let parser = pulldown_cmark::Parser::new(&md);
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html><head><meta charset=\"UTF-8\" /></head><body>");
    pulldown_cmark::html::push_html(&mut html, parser);
    html.push_str("</body></html>");
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn titles() -> std::io::Result<HttpResponse> {
    let title_map = read_title_map()?;
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html><head><meta charset=\"UTF-8\" /></head><body>");
    html.push_str("<h1>/titles</h1><ul>");
    for (title, _) in title_map.iter() {
        html.push_str(&format!(
            "<li><a href=\"{}\">{}</a></li>",
            to_title_url(title),
            title
        ));
    }
    html.push_str("</ul></body></html>");
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn title(params: web::Path<(String,)>) -> std::io::Result<HttpResponse> {
    let title_map = read_title_map()?;
    let title = &params.0;
    if let Some(page_ids) = title_map.get(title) {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html><head><meta charset=\"UTF-8\" /></head><body>");
        html.push_str(&format!("<h1>{}</h1>", to_title_url(&title)));
        html.push_str("<ul>");
        for page_id in page_ids.iter() {
            html.push_str(&format!(
                "<li><a href=\"{}\">{}</a></li>",
                to_page_url(&page_id),
                page_id.to_string()
            ));
        }
        html.push_str("</ul>");
        html.push_str("</body></html>");
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
