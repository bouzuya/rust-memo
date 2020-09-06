mod command;
mod handler;
mod handler_helpers;
mod helpers;
mod page_id;
mod page_title;
mod template;
mod url_helpers;
mod use_case;

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
        .subcommand(
            clap::SubCommand::with_name("list")
                .about("Lists memos")
                .arg(
                    clap::Arg::with_name("obsoleted")
                        .long("obsoleted")
                        .help("Prints obsoleted memos"),
                ),
        )
        .subcommand(clap::SubCommand::with_name("new").about("Creates a new memo"))
        .subcommand(clap::SubCommand::with_name("server").about("Runs server"))
        .get_matches();
    match matches.subcommand() {
        ("new", _) => crate::command::new::new()?,
        ("edit", Some(sub_matches)) => {
            let id_like_string = sub_matches.value_of("IDLike").expect("IDLike required");
            crate::command::edit::edit(id_like_string)?
        }
        ("list", Some(sub_matches)) => {
            let obsoleted = sub_matches.is_present("obsoleted");
            crate::command::list::list(obsoleted)?
        }
        ("server", _) => crate::command::server::server()?,
        _ => {}
    }
    Ok(())
}
