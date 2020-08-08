mod page_id;

use actix_web::{web, HttpResponse};
use clap;
use page_id::PageId;
use std::fs::{read_to_string, File};
use std::io::Write;

fn create_new_file(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let id = PageId::new().expect("This application is out of date.");
    let file_name = format!("{}.md", id.to_string());
    let mut file = File::create(&file_name)?;
    writeln!(file, "{}", content)?;
    file.flush()?;
    Ok(file_name)
}

fn edit_file(id_as_string: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let page_id = PageId::from_str(id_as_string).expect("invalid ID format");
    let old_file_name = format!("{}.md", page_id.to_string());
    let old = read_to_string(&old_file_name)?;
    let header = old.lines().next().unwrap_or("# ");
    let footer = format!("## Obsoletes\n\n- {}", old_file_name);
    let content = format!("{}\n\n{}", header, footer);
    let new_file_name = create_new_file(&content)?;
    Ok((old_file_name, new_file_name))
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

async fn permalink(id: web::Path<(String,)>) -> actix_web::Result<HttpResponse> {
    let content = std::fs::read_to_string(format!("./{}.md", id.0))?;
    Ok(HttpResponse::Ok().body(content))
}

#[actix_rt::main]
async fn run_server() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .route("/", web::get().to(index))
            .route("/permalinks", web::get().to(permalinks))
            .route("/permalinks/{id}", web::get().to(permalink))
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
