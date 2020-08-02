use chrono::prelude::*;
use clap::{App, Arg, SubCommand};
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("memo")
        .subcommand(SubCommand::with_name("new"))
        .subcommand(
            SubCommand::with_name("edit")
                .about("Creates a new memo that obsoletes the specified memo")
                .arg(
                    Arg::with_name("ID")
                        .help("the id of the memo to edit")
                        .required(true),
                ),
        )
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
        _ => {}
    }
    Ok(())
}
