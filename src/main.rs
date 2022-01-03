mod podcast;

use clap::Parser;
use std::sync::mpsc::channel;
use std::thread;
use spinners::{Spinner, Spinners};
use roxmltree::Document;
use crate::podcast::Item;

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
        let items = get_rss(&url);
        let _ = sender.send(items);
    });
    let _ = thread_join_handle.join();
    spinner.stop();
    let feed = receiver.recv().unwrap();
    let noi: i32 = feed.len().try_into().unwrap();
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
    for item in feed.iter().take(last + 1).skip(first){
        item.print();
    }
}


#[test]
fn test_get_rss(){
    let temporal = "!";
    assert_eq!(temporal, "");
}
fn get_rss(url: &str)->Vec<Item>{
    let mut items: Vec<Item> = Vec::new();
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    let document = Document::parse(&body).unwrap();
    let rss = document.root().first_child().unwrap();
    let channel = rss.children().find(|i| i.has_tag_name("channel")).unwrap();
    for item in channel.children().filter(|i| i.has_tag_name("item")).into_iter(){
        let title;
        let description;
        let enclosure;
        let link;
        let image;
        if let Some(value) = item.children().find(|p| p.has_tag_name("title")){
            if let Some(text) = value.text(){ title = text; }else{ title = ""; }
        }else{
            title = "";
        }
        if let Some(value) = item.children().find(|p| p.has_tag_name("description")){
            if let Some(text) = value.text(){ description = text; }else{ description = ""; }
        }else{
            description = "";
        }
        if let Some(value) = item.children().find(|p| p.has_tag_name("enclosure")){
            if let Some(attribute) = value.attributes().iter().find(|a| a.name() == "url"){
                enclosure = attribute.value();
            }else{
                enclosure = "";
            }
        }else{
            enclosure = "";
        }
        if let Some(value) = item.children().find(|p| p.has_tag_name("link")){
            if let Some(text) = value.text(){ link = text; }else{ link = ""; }
        }else{
            link = "";
        }
        if let Some(value) = item.children().find(|p| p.has_tag_name("image")){
            if let Some(attribute) = value.attributes().iter().find(|a| a.name() == "href"){
                image = attribute.value();
            }else{
                image = "";
            }
        }else{
            image = "";
        }
        let item = Item::new(title, description, enclosure, link, image);
        items.push(item);
    }
    items.reverse();
    items
}
