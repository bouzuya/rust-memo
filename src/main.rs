mod command;
mod helpers;
mod page_id;
mod page_title;
mod url_helpers;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::App::new("memo")
        .subcommand(
            clap::SubCommand::with_name("edit")
                .about("Creates a new memo that obsoletes the specified memo")
                .arg(
                    clap::Arg::with_name("IDLike")
                        .help("the id of the memo to edit")
                        .required(true),
                ),
        )
        .subcommand(clap::SubCommand::with_name("new").about("Creates a new memo"))
        .subcommand(clap::SubCommand::with_name("server").about("Runs server"))
        .get_matches();
    match matches.subcommand() {
        ("new", _) => crate::command::new::new()?,
        ("edit", Some(sub_matches)) => {
            let id_like_string: &str = sub_matches.value_of("IDLike").expect("IDLike required");
            crate::command::edit::edit(id_like_string)?
        }
        ("server", _) => crate::command::server::server()?,
        _ => {}
    }
    Ok(())
}
