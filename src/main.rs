use clap::{App, Arg, SubCommand};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .subcommand(SubCommand::with_name("version"))
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("version"){
        println!("Version: {}", VERSION);
    }
}
