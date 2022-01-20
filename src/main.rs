mod podcast;

use clap::{App, Arg, AppSettings};
use std::sync::mpsc::channel;
use std::thread;
use spinners::{Spinner, Spinners};
use crate::podcast::Podcast;

const NAME: &str =env!("CARGO_PKG_NAME");
const DESCRIPTION: &str =env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::new("debug")
             .short('d')
             .long("debug")
             .takes_value(false))
        .arg(Arg::new("json")
             .short('j')
             .long("json")
             .takes_value(false))
        .arg(Arg::new("url")
             .short('u')
             .long("url")
             .takes_value(true))
        .arg(Arg::new("first")
             .short('f')
             .long("first")
             .required(false)
             .takes_value(true))
        .arg(Arg::new("last")
             .short('l')
             .long("first")
             .required(false)
             .takes_value(true))
        .subcommand(App::new("list")
                    .about("List")
                    )
        .get_matches();
    let url = matches.value_of("url").unwrap();

    let first: i32;
    let mut last: i32;
    let (sender, receiver) = channel();
    let spinner = Spinner::new(&Spinners::Dots9,
                               "Downloading feed".to_string());
    let thread_join_handle = thread::spawn(move || {
        let podcast = Podcast::new(&url);
        let _ = sender.send(podcast);
    });
    let _ = thread_join_handle.join();
    spinner.stop();
    println!();
    let podcast = receiver.recv().unwrap();
    println!("Podcast title: {}", podcast.get_title());
    println!("Podcast description: {}", podcast.get_description());
    let items = podcast.get_episodes();
    let noi: i32 = items.len().try_into().unwrap();
    if let Some(value) = matches.value_of("first"){
        match value.parse::<i32>(){
            Ok(fvalue) => {
                if fvalue <= 0 && fvalue <= noi {
                    first = noi - fvalue;
                }else if fvalue < 0 && fvalue > noi {
                    first = 0;
                } else {
                    first = fvalue - 1;
                }
            },
            _ => first = 0,
        }
    }else{
        first = 0;
    }
    if let Some(value) = matches.value_of("last"){
        match value.parse::<i32>(){
            Ok(fvalue) => {
                if fvalue <= 0 && fvalue < noi {
                    last = noi - fvalue;
                }else if fvalue <= 0 && fvalue > noi {
                    last = noi - 1;
                } else {
                    last = fvalue - 1;
                }
            },
            _ => last = (noi - 1).try_into().unwrap(),
        }
    }else{
        last = (noi - 1).try_into().unwrap();
    }
    if last < first{
        last = first;
    }
    if let Some(_sub) = matches.subcommand_matches("list"){
        for item in items.iter().take((last + 1).try_into().unwrap()).skip(first.try_into().unwrap()){
            item.print();
        }
    }
}


#[test]
fn test_get_rss(){
    let temporal = "!";
    assert_eq!(temporal, "");
}
