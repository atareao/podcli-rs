mod podcast;

use std::process;

use clap::{App, Arg, AppSettings};
use spinners::{Spinner, Spinners};
use crate::podcast::get_rss;
use inquire::{Text, Select};
use colored::*;
use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source, Sink};
use itertools::Itertools;

const NAME: &str =env!("CARGO_PKG_NAME");
const DESCRIPTION: &str =env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");


#[tokio::main]
async fn main(){
    let matches = App::new(NAME)
        .version(VERSION)
       .author(AUTHORS)
        .about(DESCRIPTION)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::new("debug")
             .short('d')
             .long("debug")
             .takes_value(false))
        .arg(Arg::new("interactive")
             .short('i')
             .long("interactive")
             .takes_value(false))
        .arg(Arg::new("url")
             .short('u')
             .long("url")
             .required(true)
             .takes_value(true))
        .subcommand(App::new("list")
                    .about("List")
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
    let url = matches.value_of("url").unwrap();
    println!("{:?}", url);
    if matches.is_present("interactive"){
    println!("{:?}", "aqui");
        let spinner = Spinner::new(&Spinners::Dots9,
                                   "Downloading feed".to_string());
        let podcast = get_rss(url).await.unwrap();
        spinner.stop();
        let options = vec!["List episodes", "Get episode", "Play episode"];
        let ans = Select::new("Select option:", options).prompt();
        match ans {
            Ok(choice) => {
                println!("Selected {}", choice);
                if choice.contains("List episodes"){
                    for id in podcast.get_episodes().keys().sorted(){
                        let episode = podcast.get_episodes().get(id).unwrap();
                        println!("{}", episode);
                    }
                }else if choice.contains("Get episode") {
                    let ans = Select::new("Select option:", podcast.get_titles()).prompt();
                    match ans{
                        Ok(choice) => {
                            println!("{}", &choice);
                            let re = Regex::new(r"(\d*)\.").unwrap();
                            let capture = re.captures(&choice).unwrap();
                            let id = capture.get(1).map_or("", |m| m.as_str()).parse::<usize>().unwrap();
                            let episode = podcast.get_episodes().get(&id).unwrap();
                            episode.print();
                        },
                        Err(_) => println!("There was an error, please select again"),
                    }
                }else if choice.contains("Play episode") {
                    let ans = Select::new("Select option:", podcast.get_titles()).prompt();
                    match ans{
                        Ok(choice) => {
                            println!("{}", &choice);
                            let re = Regex::new(r"(\d*)\.").unwrap();
                            let capture = re.captures(&choice).unwrap();
                            let id = capture.get(1).map_or("", |m| m.as_str()).parse::<usize>().unwrap();
                            let episode = podcast.get_episodes().get(&id).unwrap();
                            episode.print();
                            let spinner = Spinner::new(&Spinners::Dots9,
                                                       "Downloading episode".to_string());
                            let filename = format!("/tmp/{}.mp3", id);
                            println!("{:?}", &filename);
                            episode.download(&filename).await;
                            spinner.message("Playing...".to_string());
                            play(&filename);
                            spinner.stop();
                        },
                        Err(_) => println!("There was an error, please select again"),
                    }
                }
            },
            Err(_) => println!("There was an error, please select again"),
        }
    }else if let Some(sub) = matches.subcommand_matches("list"){
        let url = sub.value_of("url").unwrap();

        let spinner = Spinner::new(&Spinners::Dots9,
                                   "Downloading feed".to_string());
        let podcast = get_rss(url).await.unwrap();
        let items = podcast.get_episodes();
        spinner.stop();
        for id in items.keys(){
            println!("{} {} {}",
                     "=====".cyan(),
                     id.to_string().blue(),
                     "=====".cyan());
            items.get(id).unwrap().print();
        }
        process::exit(0);
    }
}

fn play(filename: &str){
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(filename).unwrap());
    let source = Decoder::new(file).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}


#[test]
fn test_get_rss(){
    let temporal = "!";
    assert_eq!(temporal, "");
}
