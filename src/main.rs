use chrono::prelude::*;
use clap::App;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("memo").subcommand(App::new("new")).get_matches();
    if let Some(_) = matches.subcommand_matches("new") {
        let now = Utc::now();
        let file_name = format!("{}.md", now.format("%Y%m%dT%H%M%SZ"));

        println!("{}", file_name);

        let mut file = File::create(file_name)?;
        writeln!(file, "{}", "# ")?;
        file.flush()?;
    }
    Ok(())
}
