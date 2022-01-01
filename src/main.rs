use clap::Parser;
use roxmltree::{Document, Node};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

struct Item{
    title: String,
    description: String,
    link: String
}

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}

#[test]
fn get_rss(){
    let body = reqwest::blocking::get("https://atareao.es/mp3-feed/").unwrap().text().unwrap();
    //println!("body = {:?}", body);
    let document = Document::parse(&body).unwrap();
    let rss = document.root().first_child().unwrap();
    for child in rss.children(){
        println!("Child name: {}", child.tag_name().name());
    }
    let channel = rss.children().find(|i| i.has_tag_name("channel")).unwrap();
    println!("Channel: '{}'", channel.tag_name().name());
    for item in channel.children().filter(|i| i.has_tag_name("item")).into_iter(){
        let title = item.children().find(|p| p.has_tag_name("title")).unwrap().text().unwrap();
        let numero = item.children().find(|p| p.has_tag_name("itunes:episode")).unwrap().text().unwrap();
        println!("El título del episodio {} es: {}", numero, title);
    }
    let temporal = "!";
    assert_eq!(temporal, "");
}
