mod page_id;

use actix_web::{web, HttpResponse};
use clap;
use page_id::PageId;
use std::fs::{read_to_string, File};
use std::io::Write;

fn to_file_name(page_id: &PageId) -> String {
    format!("{}.md", page_id.to_string())
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
    let old = read_to_string(&old_file_name)?;
    let header = old.lines().next().unwrap_or("# ");
    let footer = format!(
        "## Obsoletes\n\n- [{}](/pages/{})",
        page_id.to_string(),
        page_id.to_string()
    );
    let content = format!("{}\n\n{}", header, footer);
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
                let id_as_string = page_id.to_string();
                format!(
                    "<li><a href=\"/pages/{}\">{}</a></li>",
                    id_as_string, id_as_string
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

#[actix_rt::main]
async fn run_server() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .route("/", web::get().to(index))
            .route("/pages", web::get().to(pages))
            .route("/pages/{id}", web::get().to(page))
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
