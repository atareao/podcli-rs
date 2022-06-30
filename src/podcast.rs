use roxmltree::Document;
use colored::*;
use reqwest::Error;

pub struct Item{
    title: String,
    description: String,
    enclosure: String,
    link: String,
    image: String,
}

pub struct Podcast{
    url: String,
    title: String,
    description: String,
    link: String,
    image: String,
    items: Vec<Item>,
}

impl Podcast{
    pub async fn new(url: &str)->Result<Podcast, Error>{
        get_rss(&url).await
    }

    pub fn get_episodes(&self)->&Vec<Item>{
        &self.items
    }

    pub fn get_title(&self)->&str{
        &self.title
    }

    pub fn get_description(&self)->&str{
        &self.description
    }

    async fn get_raw_rss(url: &str) -> Result<String, Error>{
        return reqwest::get(url)
            .await
            .unwrap()
            .text()
            .await
    }
}

pub async fn get_rss(url: &str)->Result<Podcast, Error>{
    let url = url.to_string();
    let mut description = "".to_string();
    let mut title = "".to_string();
    let mut link = "".to_string();
    let mut image = "".to_string();
    let mut items: Vec<Item> = Vec::new();
    let body = reqwest::get(&url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let document = Document::parse(&body).unwrap();
    let rss = document.root().first_child().unwrap();
    let channel = rss.children().find(|i| i.has_tag_name("channel")).unwrap();
    /*
     * title
     * description
     * link
     * image / url
     */
    if let Some(node) = channel.children().find(|p| p.has_tag_name("description")){
        if let Some(text) = node.text(){
            description = text.to_string();
        }
    }
    if let Some(node) = channel.children().find(|p| p.has_tag_name("title")){
        if let Some(text) = node.text(){
            title = text.to_string();
        }
    }
    if let Some(node) = channel.children().find(|p| p.has_tag_name("link")){
        if let Some(text) = node.text(){
            link = text.to_string();
        }
    }
    if let Some(node) = channel.children().find(|p| p.has_tag_name("image")){
        if let Some(text) = node.text(){
            image = text.to_string();
        }
    }
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
    Ok(Podcast{
        url,
        title,
        description,
        link,
        image,
        items,
    })
}

impl Item{
    pub fn new(title: &str, description: &str, enclosure: &str, link: &str,
               image: &str)->Item{
        Self{title: title.to_string(),
             description: description.to_string(),
             enclosure: enclosure.to_string(),
             link: link.to_string(),
             image: image.to_string()}
    }

    pub fn get_title(&self)->&str{
        &self.title
    }

    pub fn get_descrption(&self)->&str{
        &self.description
    }

    pub fn get_enclosure(&self)->&str{
        &self.enclosure
    }

    pub fn get_link(&self)->&str{
        &self.link
    }

    pub fn print(&self){
        println!("{}: {}", "Title".red(), self.title.blue());
        println!("{}: {}", "Description".red(), self.description);
        println!("{}: {}", "Enclosure".red(), self.enclosure.magenta());
        println!("{}: {}", "Link".red(), self.link);
        println!("{}: {}", "Image".red(), self.image);
    }
}

