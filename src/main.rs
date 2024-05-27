mod podcast;

use crate::podcast::Podcast;
use std::{env, process, str::FromStr};

use crate::podcast::get_rss;
use colored::*;
use inquire::Select;
use itertools::Itertools;
use regex::Regex;
use rodio::{Decoder, OutputStream, Sink};
use spinners::{Spinner, Spinners};
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter
};
use tracing::{debug, error, info};
use std::fs::File;
use std::io::BufReader;
use clap::{Parser, Args, Subcommand, ArgAction};

const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[derive(Parser)]
#[command(author = AUTHORS, version = VERSION, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = ArgAction::SetTrue)]
    /// debug podcli
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    List(ListArgs),
    /// Run interactive mode
    Interactive(InteractiveArgs),
}

#[derive(Args)]
struct ListArgs{
    #[arg(short, long)]
    url: String,

    #[arg(short, long)]
    first: Option<String>,

    #[arg(short, long)]
    last: Option<String>,

    #[arg(short, long)]
    json: Option<bool>,

}

#[derive(Args)]
struct InteractiveArgs{
    #[arg(short, long)]
    url: String,
}

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("DEBUG".to_string());

    tracing_subscriber::registry()
        .with(EnvFilter::from_str(&log_level).unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();

    debug!("log_level: {}", log_level);

    let cli = Cli::parse();
    match &cli.command{
        Commands::List(args) => {
            let url = &args.url;
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
        },
        Commands::Interactive(args) =>{
            let url = &args.url;
            let mut podcast = get_podcast(url).await;
            loop {
                interactive(&mut podcast, url).await;
            }

        }
    }
    process::exit(0);
}

fn play(filename: &str) {
    debug!("play: {}", filename);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(filename).unwrap());
    let source = Decoder::new(file).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}

async fn get_podcast(url: &str) -> Podcast{
    info!("get_podcast: {}", url);
    let mut spinner = Spinner::new(Spinners::Dots9, "Downloading feed".to_string());
    let podcast = get_rss(url).await.unwrap();
    spinner.stop();
    podcast
}

async fn interactive(podcast: &mut Podcast, url: &str) {
    let options = vec![
        "1. List episodes",
        "2. Get episode",
        "3. Play episode",
        "4. Reload",
        "5. Exit",
    ];
    let ans = Select::new("Select option:", options).prompt_skippable();
    match ans {
        Ok(response) => {
            let choice = response.unwrap_or("Exit");
            println!("Selected {}", choice);
            if choice.contains("List episodes") {
                for id in podcast.get_episodes().keys().sorted() {
                    let episode = podcast.get_episodes().get(id).unwrap();
                    println!("{}", episode);
                }
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
                        match episode.download(&filename).await{
                            Ok(result) => {
                                debug!("Downloaded: {}", result);
                            },
                            Err(e) => error!("Can not download by: {}", e),
                        }
                        play(&filename);
                        spinner.stop();
                    }
                    Err(_) => println!("There was an error, please select again"),
                }
            } else if choice.contains("Reload") {
                let mut spinner = Spinner::new(Spinners::Dots9, "Downloading feed".to_string());
                *podcast = get_rss(url).await.unwrap();
                spinner.stop();
            } else if choice.contains("Exit") {
                process::exit(0);
            }
        }
        Err(e) => println!("There was an error, please select again: {e}"),
    }
}

#[test]
fn test_get_rss() {
    let temporal = "!";
    assert_eq!(temporal, "");
}
