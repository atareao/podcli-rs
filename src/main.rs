mod podcast;

use clap::{App, Arg, AppSettings, ArgMatches};
use std::sync::mpsc::channel;
use std::thread;
use spinners::{Spinner, Spinners};
use crate::podcast::Podcast;
use colored::*;

const NAME: &str =env!("CARGO_PKG_NAME");
const DESCRIPTION: &str =env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main(){
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::new("debug")
             .short('d')
             .long("debug")
             .takes_value(false))
        .subcommand(App::new("list")
                    .about("List")
                    .arg(Arg::new("url")
                         .short('u')
                         .long("url")
                         .required(true)
                         .takes_value(true))
                    .arg(Arg::new("first")
                         .short('f')
                         .long("first")
                         .required(false)
                         .takes_value(true))
                    .arg(Arg::new("last")
                         .short('l')
                         .long("last")
                         .required(false)
                         .takes_value(true))
                    .arg(Arg::new("json")
                         .short('j')
                         .long("json")
                         .takes_value(false))
                    )
        .get_matches();
    if let Some(sub) = matches.subcommand_matches("list"){
        let url = sub.value_of("url").unwrap();
        let first_string = sub.value_of("first");
        let last_string = sub.value_of("last");

        let podcast = get_podcast(url.to_string());
        let items = podcast.get_episodes();
        let total: i32 = items.len().try_into().unwrap();
        let first = calc_first(first_string, total);
        let mut last = calc_last(last_string, total);

        if last < first{
            last = first;
        }

        for (index, item) in items.iter().take((last + 1).try_into().unwrap()).skip(first.try_into().unwrap()).enumerate(){
            println!("{} {} {}",
                     "=====".cyan(),
                     index.to_string().blue(),
                     "=====".cyan());
            item.print();
        }
    }
}

async fn get_podcast(url: String) -> Podcast{
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
    receiver.recv().unwrap()
}

fn calc_first(first_value: Option<&str>, total: i32) -> i32{
    if let Some(value) = first_value{
        match value.parse::<i32>(){
            Ok(fvalue) => {
                if fvalue <= 0 && fvalue <= total {
                    return total - fvalue;
                }else if fvalue < 0 && fvalue > total {
                    return 0;
                } else {
                    return fvalue - 1;
                }
            },
            _ => return 0,
        }
    }
    0
}

fn calc_last(last_value: Option<&str>, total: i32) -> i32{
    if let Some(value) = last_value{
        match value.parse::<i32>(){
            Ok(fvalue) => {
                if fvalue <= 0 && fvalue < total {
                    return total - fvalue;
                }else if fvalue <= 0 && fvalue > total {
                    return total - 1;
                } else {
                    return fvalue - 1;
                }
            },
            _ => return total - 1,
        }
    }
    total - 1
}


#[test]
fn test_get_rss(){
    let temporal = "!";
    assert_eq!(temporal, "");
}
