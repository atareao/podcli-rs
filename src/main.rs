mod podcast;

use crate::podcast::Episode;
use crate::podcast::Podcast;
use std::{env, process, str::FromStr};

use crate::podcast::get_rss;
use clap::{ArgAction, Args, Parser, Subcommand};
use colored::*;
use inquire::Select;
use itertools::Itertools;
use regex::Regex;
use rodio::{Decoder, DeviceSinkBuilder, Player};
use spinners::{Spinner, Spinners};
use std::fs::File;
use std::io;
use std::io::BufReader;
use tracing::{debug, error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[derive(Parser)]
#[command(author = AUTHORS, version = VERSION, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = ArgAction::SetTrue)]
    /// debug podcli
    debug: bool,

    #[arg(long, global = true, value_name = "DIR")]
    /// Directory to store downloaded episodes
    download_dir: Option<String>,

    #[arg(long, global = true, value_name = "SPEED", default_value_t = 1.0)]
    /// Playback speed (1.0 = normal, 0.5 = half, 2.0 = double)
    speed: f32,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List episodes from an RSS feed
    List(ListArgs),
    /// Run interactive mode
    Interactive(InteractiveArgs),
}

#[derive(Args)]
struct ListArgs {
    #[arg(short, long)]
    url: String,

    #[arg(short, long, conflicts_with = "last")]
    first: Option<usize>,

    #[arg(short, long)]
    last: Option<usize>,

    #[arg(short, long, action = ArgAction::SetTrue)]
    json: bool,
}

#[derive(Args)]
struct InteractiveArgs {
    #[arg(short, long)]
    url: String,
}

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("DEBUG".to_string());

    tracing_subscriber::registry()
        .with(EnvFilter::from_str(&log_level).unwrap())
        .with(tracing_subscriber::fmt::layer().with_writer(io::stderr))
        .init();

    debug!("log_level: {}", log_level);

    let cli = Cli::parse();
    match &cli.command {
        Commands::List(args) => {
            let url = &args.url;
            let mut spinner = Spinner::new(Spinners::Dots9, "Downloading feed".to_string());
            let podcast = get_rss(url).await.unwrap();
            let episodes: Vec<&Episode> = podcast.get_episodes().values().collect();
            let selected = select(&episodes, args.first, args.last);
            spinner.stop();
            if args.json {
                println!("{}", to_json(&selected));
            } else {
                for episode in selected {
                    println!(
                        "{} {} {}",
                        "=====".cyan(),
                        episode.get_id().to_string().blue(),
                        "=====".cyan()
                    );
                    episode.print();
                }
            }
        }
        Commands::Interactive(args) => {
            let url = &args.url;
            let mut podcast = get_podcast(url).await;
            let download_dir = cli.download_dir.clone();
            let speed = cli.speed;
            loop {
                interactive(&mut podcast, url, download_dir.as_deref(), speed).await;
            }
        }
    }
    process::exit(0);
}

fn play(filename: &str, speed: f32) {
    debug!("play: {}", filename);
    let mut device_sink = DeviceSinkBuilder::open_default_sink().unwrap();
    device_sink.log_on_drop(false);
    let file = BufReader::new(File::open(filename).unwrap());
    let player = Player::connect_new(device_sink.mixer());
    player.set_speed(speed);
    let source = Decoder::new(file).unwrap();
    player.append(source);
    player.sleep_until_end();
}

async fn get_podcast(url: &str) -> Podcast {
    info!("get_podcast: {}", url);
    let mut spinner = Spinner::new(Spinners::Dots9, "Downloading feed".to_string());
    let podcast = get_rss(url).await.unwrap();
    spinner.stop();
    podcast
}

async fn interactive(podcast: &mut Podcast, url: &str, download_dir: Option<&str>, speed: f32) {
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
                        println!("{}", choice);
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
                        println!("{}", choice);
                        let re = Regex::new(r"(\d*)\.").unwrap();
                        let capture = re.captures(&choice).unwrap();
                        let id = capture
                            .get(1)
                            .map_or("", |m| m.as_str())
                            .parse::<usize>()
                            .unwrap();
                        let episode = podcast.get_episodes().get(&id).unwrap();
                        episode.print();
                        let dir = download_dir.unwrap_or("/tmp");
                        let filename = format!("{}/{}.mp3", dir, id);
                        println!("{:?}", filename);
                        match episode.download(&filename).await {
                            Ok(result) => {
                                debug!("Downloaded: {}", result);
                            }
                            Err(e) => error!("Can not download by: {}", e),
                        }
                        play(&filename, speed);
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

/// Select the episodes to list: descending id (newest first), then apply
/// `--first` (keep the newest N) or `--last` (keep the oldest N).
fn select<'a>(
    episodes: &[&'a Episode],
    first: Option<usize>,
    last: Option<usize>,
) -> Vec<&'a Episode> {
    let mut ordered: Vec<&Episode> = episodes.to_vec();
    ordered.sort_by(|a, b| b.get_id().cmp(a.get_id())); // descending id = newest first
    if let Some(n) = first {
        ordered.truncate(n);
    } else if let Some(n) = last {
        if n < ordered.len() {
            ordered = ordered.split_off(ordered.len() - n);
        }
    }
    ordered
}

/// Serialize the selected episodes as a single JSON array string.
fn to_json(episodes: &[&Episode]) -> String {
    serde_json::to_string(episodes).unwrap()
}

#[test]
fn test_get_rss() {
    assert_eq!(1, 1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    fn sample_episodes() -> Vec<Episode> {
        vec![
            Episode::new(1, "one", "d1", "e1", "l1", "i1", "10:00"),
            Episode::new(2, "two", "d2", "e2", "l2", "i2", "20:00"),
            Episode::new(3, "three", "d3", "e3", "l3", "i3", "30:00"),
        ]
    }

    fn refs(episodes: &[Episode]) -> Vec<&Episode> {
        episodes.iter().collect()
    }

    fn ids(episodes: &[&Episode]) -> Vec<usize> {
        episodes.iter().map(|e| *e.get_id()).collect()
    }

    // 2.1 select(): --first N returns the N highest ids, descending.
    #[test]
    fn select_first_returns_highest_ids_descending() {
        let eps = sample_episodes();
        let selected = select(&refs(&eps), Some(2), None);
        assert_eq!(ids(&selected), vec![3, 2]);
    }

    // 2.2 select(): --last N returns the N lowest ids, descending.
    #[test]
    fn select_last_returns_lowest_ids_descending() {
        let eps = sample_episodes();
        let selected = select(&refs(&eps), None, Some(2));
        assert_eq!(ids(&selected), vec![2, 1]);
    }

    // 2.3 select(): no filter -> full descending order; first == 0 -> none;
    // first greater than feed size -> all.
    #[test]
    fn select_ordering_and_edge_cases() {
        let eps = sample_episodes();

        let all = select(&refs(&eps), None, None);
        assert_eq!(ids(&all), vec![3, 2, 1]);

        let none = select(&refs(&eps), Some(0), None);
        assert!(none.is_empty(), "first=0 must yield no episodes");

        let over = select(&refs(&eps), Some(10), None);
        assert_eq!(ids(&over), vec![3, 2, 1]);
    }

    // 2.4 clap contract: --first abc is rejected.
    #[test]
    fn clap_first_rejects_non_numeric() {
        let result = Cli::try_parse_from(["podcli", "list", "--url", "x", "--first", "abc"]);
        assert!(
            result.is_err(),
            "--first abc must be rejected: {:?}",
            result.err()
        );
    }

    // 2.5 clap contract: --first + --last together is rejected.
    #[test]
    fn clap_first_and_last_are_mutually_exclusive() {
        let result = Cli::try_parse_from([
            "podcli", "list", "--url", "x", "--first", "2", "--last", "3",
        ]);
        assert!(
            result.is_err(),
            "--first and --last must conflict: {:?}",
            result.err()
        );
    }

    // 2.6 clap contract: bare --json parses with no value required.
    #[test]
    fn clap_json_flag_takes_no_value() {
        let result = Cli::try_parse_from(["podcli", "list", "--url", "x", "--json"]);
        assert!(
            result.is_ok(),
            "bare --json must parse with no value: {:?}",
            result.err()
        );
    }

    // 2.7 --json output is a single valid JSON array.
    #[test]
    fn json_output_is_a_single_valid_array() {
        let eps = sample_episodes();
        let json = to_json(&refs(&eps));
        let value: serde_json::Value =
            serde_json::from_str(&json).expect("--json output must parse as JSON");
        assert!(value.is_array(), "top-level JSON value must be an array");
    }

    // 2.8 JSON objects contain the full field set with correct value types.
    #[test]
    fn json_objects_have_expected_fields() {
        let eps = sample_episodes();
        let json = to_json(&refs(&eps));
        let value: serde_json::Value =
            serde_json::from_str(&json).expect("--json output must parse as JSON");
        let array = value
            .as_array()
            .expect("top-level JSON value must be an array");
        assert_eq!(array.len(), 3);

        for element in array {
            let object = element.as_object().expect("each element must be an object");
            for key in [
                "id",
                "title",
                "description",
                "enclosure",
                "link",
                "image",
                "duration",
            ] {
                assert!(object.contains_key(key), "missing JSON key {key}");
            }
            assert!(object["id"].is_number(), "id must be a number");
            for key in [
                "title",
                "description",
                "enclosure",
                "link",
                "image",
                "duration",
            ] {
                assert!(object[key].is_string(), "{key} must be a string");
            }
        }
    }

    // 2.9 --json combined with --first yields an array of exactly N episodes.
    #[test]
    fn json_with_first_yields_filtered_array() {
        let eps = sample_episodes();
        let selected = select(&refs(&eps), Some(2), None);
        let json = to_json(&selected);
        let value: serde_json::Value =
            serde_json::from_str(&json).expect("--json output must parse as JSON");
        let array = value
            .as_array()
            .expect("top-level JSON value must be an array");
        assert_eq!(array.len(), 2, "--json --first 2 must yield two episodes");
    }

    // 1.2 Characterization: current human-readable non-JSON header is
    // `===== <id> =====` followed by `Episode::print`, and `Display` for an
    // episode is `<id>. <title> [<duration>]`.
    #[test]
    fn characterization_human_readable_list_format() {
        use colored::Colorize;

        let eps = sample_episodes();
        let episode = &eps[0];

        let header = format!(
            "{} {} {}",
            "=====".cyan(),
            episode.get_id().to_string().blue(),
            "=====".cyan()
        );
        assert_eq!(header, "===== 1 =====");

        assert_eq!(episode.to_string(), "1. one [10:00]");
    }

    // 1.1 clap contract: global options accepted AFTER the interactive subcommand.
    #[test]
    fn clap_interactive_accepts_download_dir_after_subcommand() {
        let result = Cli::try_parse_from([
            "podcli",
            "interactive",
            "--url",
            "x",
            "--download-dir",
            "/tmp",
        ]);
        assert!(
            result.is_ok(),
            "--download-dir after subcommand must parse: {:?}",
            result.err()
        );
    }

    // 1.2 clap contract: global --speed accepted AFTER the interactive subcommand.
    #[test]
    fn clap_interactive_accepts_speed_after_subcommand() {
        let result = Cli::try_parse_from(["podcli", "interactive", "--url", "x", "--speed", "1.5"]);
        assert!(
            result.is_ok(),
            "--speed after subcommand must parse: {:?}",
            result.err()
        );
    }

    // 1.3 clap contract: global --download-dir accepted after the list subcommand.
    #[test]
    fn clap_list_accepts_download_dir_after_subcommand() {
        let result =
            Cli::try_parse_from(["podcli", "list", "--url", "x", "--download-dir", "/tmp"]);
        assert!(
            result.is_ok(),
            "--download-dir after list must parse: {:?}",
            result.err()
        );
    }

    // 1.4 help text: `list` describes listing episodes, not a placeholder.
    #[test]
    fn list_help_describes_listing_episodes() {
        use clap::CommandFactory;

        let list = Cli::command()
            .find_subcommand("list")
            .expect("list subcommand must exist")
            .get_about()
            .map(|about| about.to_string())
            .unwrap_or_default();
        let about = list.to_lowercase();
        assert!(
            about.contains("episodes"),
            "list help must mention episodes, got: {list:?}"
        );
        assert!(
            !about.contains("myapp"),
            "list help must not mention myapp, got: {list:?}"
        );
    }
}
