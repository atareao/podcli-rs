mod podcast;

use clap::Parser;
use std::sync::mpsc::channel;
use std::thread;
use spinners::{Spinner, Spinners};
use crate::podcast::Podcast;
use std::io;
use std::fs::File;

/*
const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
*/

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Debug mode
    #[clap(short='D', long)]
    debug: bool,

    /// Show the title of the episode(s)
    #[clap(short, long)]
    title: bool,

    /// Show the description of the episode(s)
    #[clap(short, long)]
    description: bool,

    /// The url of the feed of the podcast. Required.
    #[clap(short, long)]
    url: String,

    /// The first episode to adquire. If none, default is the first.
    #[clap(short='F', long)]
    first: Option<i32>,

    /// The last episode to adquire. If none, default is the last.
    #[clap(short='L', long)]
    last: Option<i32>,

    /// Download the picture of the episode(s)
    #[clap(short, long)]
    image: bool,

    /// Download the audio of the episode(s)
    #[clap(short, long)]
    audio: bool,

    /// Download the feed
    #[clap(short, long)]
    feed: bool,

    /// Play the audio of the episode(s)
    #[clap(short, long)]
    play: bool,

}

fn main() {
    let args = Args::parse();
    let url = args.url;
    let first: usize;
    let mut last: usize;
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
    if let Some(value) = args.first{
        if value <= 0 && value <= noi {
            first = (noi - value).try_into().unwrap();
        }else if value < 0 && value > noi {
            first = 0;
        } else {
            first = (value - 1).try_into().unwrap();
        }
    }else{
        first = 0;
    }
    if let Some(value) = args.last{
        if value <= 0 && value < noi {
            last = (noi - value).try_into().unwrap();
        }else if value <= 0 && value > noi {
            last = (noi - 1).try_into().unwrap();
        } else {
            last = (value - 1).try_into().unwrap();
        }
    }else{
        last = (noi - 1).try_into().unwrap();
    }
    if last < first{
        last = first;
    }
    /*
    let mut handles = Vec::new();
    let spinner = Spinner::new(&Spinners::Dots9,
                               "Downloading feed".to_string());
                               */
    for item in items.iter().take(last + 1).skip(first){
        item.print();
        /*
        let thread_join_handle = thread::spawn(move || {
            let mut resp = reqwest::blocking::get(item.get_enclosure()).expect("Failed download");
            let mut out = File::create("archivo.mp3").expect("Failed file");
            io::copy(&mut resp, &mut out).expect("Failed copy");
        });
        handles.push(thread_join_handle);
        */
    }
    /*
    for thread in handles{
        let _ = thread.join();
    }
    spinner.stop();
    */
}


#[test]
fn test_get_rss(){
    let temporal = "!";
    assert_eq!(temporal, "");
}
