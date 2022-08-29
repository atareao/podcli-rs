use colored::*;
use html2md::parse_html;
use itertools::Itertools;
use reqwest::Error;
use roxmltree::Document;
use std::{collections::HashMap, fmt, fs::File, io::Cursor};
use termimad::{self, MadSkin};

pub struct Episode {
    id: usize,
    title: String,
    description: String,
    enclosure: String,
    link: String,
    image: String,
}

impl fmt::Display for Episode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}. {}", self.id, self.title)
    }
}

pub struct Podcast {
    url: String,
    title: String,
    description: String,
    link: String,
    image: String,
    episodes: HashMap<usize, Episode>,
}

impl Podcast {
    pub async fn new(url: &str) -> Result<Podcast, Error> {
        get_rss(url).await
    }

    pub fn get_episodes(&self) -> &HashMap<usize, Episode> {
        &self.episodes
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    async fn get_raw_rss(url: &str) -> Result<String, Error> {
        return reqwest::get(url).await.unwrap().text().await;
    }

    pub fn get_titles(&self) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        for id in self.episodes.keys().sorted().rev() {
            let episode = self.episodes.get(id).unwrap();
            ans.push(episode.to_string())
        }
        ans
    }
}

pub async fn get_rss(url: &str) -> Result<Podcast, Error> {
    let url = url.to_string();
    let mut description = "".to_string();
    let mut title = "".to_string();
    let mut link = "".to_string();
    let mut image = "".to_string();
    let mut episodes: HashMap<usize, Episode> = HashMap::new();
    let body = reqwest::get(&url).await.unwrap().text().await.unwrap();
    let document = Document::parse(&body).unwrap();
    let rss = document.root().first_child().unwrap();
    let channel = rss.children().find(|i| i.has_tag_name("channel")).unwrap();
    /*
     * title
     * description
     * link
     * image / url
     */
    if let Some(node) = channel.children().find(|p| p.has_tag_name("description")) {
        if let Some(text) = node.text() {
            description = parse_html(text);
        }
    }
    if let Some(node) = channel.children().find(|p| p.has_tag_name("title")) {
        if let Some(text) = node.text() {
            title = text.to_string();
        }
    }
    if let Some(node) = channel.children().find(|p| p.has_tag_name("link")) {
        if let Some(text) = node.text() {
            link = text.to_string();
        }
    }
    if let Some(node) = channel.children().find(|p| p.has_tag_name("image")) {
        if let Some(text) = node.text() {
            image = text.to_string();
        }
    }
    let chapters = channel.children().filter(|i| i.has_tag_name("item"));
    let total = chapters.clone().count();
    for (index, item) in chapters.enumerate() {
        let id = total - index;
        let title;
        let description;
        let enclosure;
        let link;
        let image;
        if let Some(value) = item.children().find(|p| p.has_tag_name("title")) {
            if let Some(text) = value.text() {
                title = text;
            } else {
                title = "";
            }
        } else {
            title = "";
        }
        if let Some(value) = item.children().find(|p| p.has_tag_name("description")) {
            if let Some(text) = value.text() {
                description = parse_html(text);
            } else {
                description = "".to_string();
            }
        } else {
            description = "".to_string();
        }
        if let Some(value) = item.children().find(|p| p.has_tag_name("enclosure")) {
            if let Some(attribute) = value.attributes().iter().find(|a| a.name() == "url") {
                enclosure = attribute.value();
            } else {
                enclosure = "";
            }
        } else {
            enclosure = "";
        }
        if let Some(value) = item.children().find(|p| p.has_tag_name("link")) {
            if let Some(text) = value.text() {
                link = text;
            } else {
                link = "";
            }
        } else {
            link = "";
        }
        if let Some(value) = item.children().find(|p| p.has_tag_name("image")) {
            if let Some(attribute) = value.attributes().iter().find(|a| a.name() == "href") {
                image = attribute.value();
            } else {
                image = "";
            }
        } else {
            image = "";
        }
        println!("{} - {}", id, title);
        let episode = Episode::new(id, title, &description, enclosure, link, image);
        episodes.insert(id, episode);
    }
    Ok(Podcast {
        url,
        title,
        description,
        link,
        image,
        episodes,
    })
}

impl Episode {
    pub fn new(
        id: usize,
        title: &str,
        description: &str,
        enclosure: &str,
        link: &str,
        image: &str,
    ) -> Self {
        Self {
            id,
            title: title.to_string(),
            description: description.to_string(),
            enclosure: enclosure.to_string(),
            link: link.to_string(),
            image: image.to_string(),
        }
    }

    pub fn get_id(&self) -> &usize {
        &self.id
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_descrption(&self) -> &str {
        &self.description
    }

    pub fn get_enclosure(&self) -> &str {
        &self.enclosure
    }

    pub fn get_link(&self) -> &str {
        &self.link
    }

    pub fn print(&self) {
        println!("{}: {}", "Title".red(), self.title.blue());
        println!("{}:", "Description".red());

        let mut skin = MadSkin::default();
        skin.bold.set_fg(termimad::gray(19));
        skin.set_headers_fg(termimad::rgb(255, 255, 0));
        eprintln!("{}", skin.term_text(self.get_descrption()));

        println!("{}: {}", "Enclosure".red(), self.enclosure.magenta());
        println!("{}: {}", "Link".red(), self.link);
        println!("{}: {}", "Image".red(), self.image);
    }

    pub async fn download(&self, filename: &str) -> Result<bool, Error> {
        let response = reqwest::get(&self.enclosure).await?;
        let mut content = Cursor::new(response.bytes().await?);
        let mut file = File::create(&filename).unwrap();
        std::io::copy(&mut content, &mut file).unwrap();
        file.sync_all();
        Ok(true)
    }
}
