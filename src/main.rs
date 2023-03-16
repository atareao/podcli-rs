mod podcast;

use crate::podcast::Podcast;
use std::process;

use crate::podcast::get_rss;
use clap::{App, AppSettings, Arg};
use colored::*;
use inquire::{Select, Text};
use itertools::Itertools;
use regex::Regex;
use rodio::{source::Source, Decoder, OutputStream, Sink};
use spinners::{Spinner, Spinners};
use std::fs::File;
use std::io::BufReader;

const NAME: &str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[tokio::main]
async fn main() {
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .takes_value(false),
        )
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .takes_value(false),
        )
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .required(true)
                .takes_value(true),
        )
        .subcommand(
            App::new("list")
                .about("List")
                .arg(
                    Arg::new("first")
                        .short('f')
                        .long("first")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("last")
                        .short('l')
                        .long("last")
                        .required(false)
                        .takes_value(true),
                )
                .arg(Arg::new("json").short('j').long("json").takes_value(false)),
        )
        .get_matches();
    let url = matches.value_of("url").unwrap();
    if matches.is_present("interactive") {
        let mut spinner = Spinner::new(Spinners::Dots9, "Downloading feed".to_string());
        let podcast = get_rss(url).await.unwrap();
        spinner.stop();
        loop {
            interactive(&podcast).await;
        }
    } else if let Some(sub) = matches.subcommand_matches("list") {
        let url = sub.value_of("url").unwrap();

        let mut spinner = Spinner::new(Spinners::Dots9, "Downloading feed".to_string());
        let podcast = get_rss(url).await.unwrap();
        let items = podcast.get_episodes();
        spinner.stop();
        for id in items.keys() {
            println!(
                "{} {} {}",
                "=====".cyan(),
                id.to_string().blue(),
                "=====".cyan()
            );
            items.get(id).unwrap().print();
        }
        process::exit(0);
    }
}

fn play(filename: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(filename).unwrap());
    let source = Decoder::new(file).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}

async fn interactive(podcast: &Podcast) {
    let options = vec![
        "1. List episodes",
        "2. Get episode",
        "3. Play episode",
        "4. Exit",
    ];
    let ans = Select::new("Select option:", options).prompt();
    match ans {
        Ok(choice) => {
            println!("Selected {}", choice);
            if choice.contains("List episodes") {
                for id in podcast.get_episodes().keys().sorted() {
                    let episode = podcast.get_episodes().get(id).unwrap();
                    println!("{}", episode);
                }
            } else if choice.contains("Exit") {
                process::exit(0);
            } else if choice.contains("Get episode") {
                let ans = Select::new("Select option:", podcast.get_titles()).prompt();
                match ans {
                    Ok(choice) => {
                        println!("{}", &choice);
                        let re = Regex::new(r"(\d*)\.").unwrap();
                        let capture = re.captures(&choice).unwrap();
                        let id = capture
                            .get(1)
                            .map_or("", |m| m.as_str())
                            .parse::<usize>()
                            .unwrap();
                        let episode = podcast.get_episodes().get(&id).unwrap();
                        episode.print();
                    }
                    Err(_) => println!("There was an error, please select again"),
                }
            } else if choice.contains("Play episode") {
                let ans = Select::new("Select option:", podcast.get_titles()).prompt();
                match ans {
                    Ok(choice) => {
                        println!("{}", &choice);
                        let re = Regex::new(r"(\d*)\.").unwrap();
                        let capture = re.captures(&choice).unwrap();
                        let id = capture
                            .get(1)
                            .map_or("", |m| m.as_str())
                            .parse::<usize>()
                            .unwrap();
                        let episode = podcast.get_episodes().get(&id).unwrap();
                        episode.print();
                        let mut spinner =
                            Spinner::new(Spinners::Dots9, "Downloading episode".to_string());
                        let filename = format!("/tmp/{}.mp3", id);
                        println!("{:?}", &filename);
                        episode.download(&filename).await;
                        play(&filename);
                        spinner.stop();
                    }
                    Err(_) => println!("There was an error, please select again"),
                }
            }
        }
        Err(_) => println!("There was an error, please select again"),
    }
}

#[test]
fn test_get_rss() {
    let temporal = "!";
    assert_eq!(temporal, "");
}
