use actix_web::{web, HttpResponse};
use chrono::prelude::*;
use clap;
use std::fs::{read_to_string, File};
use std::io::Write;

fn create_new_file(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let now = Utc::now();
    let file_name = format!("{}.md", now.format("%Y%m%dT%H%M%SZ"));
    let mut file = File::create(&file_name)?;
    writeln!(file, "{}", content)?;
    file.flush()?;
    Ok(file_name)
}

async fn index() -> impl actix_web::Responder {
    HttpResponse::Found()
        .header(actix_web::http::header::LOCATION, format!("/permalinks"))
        .finish()
}

async fn permalinks() -> actix_web::Result<HttpResponse> {
    let mut entries = std::fs::read_dir(".")?
        .map(|res| res.map(|e| e.path().to_str().unwrap().to_owned()))
        .collect::<Result<Vec<String>, std::io::Error>>()?;
    entries.sort();
    let s = entries.join("\n");
    Ok(HttpResponse::Ok().body(s))
}

#[actix_rt::main]
async fn run_server() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .route("/", web::get().to(index))
            .route("/permalinks", web::get().to(permalinks))
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
            let file_name = create_new_file("# ")?;
            println!("{}", file_name);
        }
        ("edit", Some(sub_matches)) => {
            if let Some(id) = sub_matches.value_of("ID") {
                let old_file_name = format!("{}.md", id);
                let old = read_to_string(&old_file_name)?;
                let header = old.lines().next().unwrap_or("# ");
                let footer = format!("## Obsoletes\n\n- {}", old_file_name);
                let content = format!("{}\n\n{}", header, footer);
                let new_file_name = create_new_file(&content)?;
                println!("{} -> {}", old_file_name, new_file_name);
            }
        }
        ("server", _) => {
            run_server()?;
        }
        _ => {}
    }
    Ok(())
}
