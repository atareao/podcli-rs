use roxmltree::Document;
use colored::*;

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
    pub fn new(url: &str)->Podcast{
        let mut podcast = Podcast{
            url: url.to_string(),
            title: String::from(""),
            description: String::from(""),
            link: String::from(""),
            image: String::from(""),
            items: Vec::new(),
        };
        let _items = podcast.get_rss(url);
        podcast
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

    fn get_rss(&mut self, url: &str){
        self.url = url.to_string();
        self.items = Vec::new();
        let body = reqwest::blocking::get(url).unwrap().text().unwrap();
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
            if let Some(text) = node.text(){ self.description = text.to_string(); }else{ self.description = "".to_string(); }
        }else{
            self.description = "".to_string();
        }
        if let Some(node) = channel.children().find(|p| p.has_tag_name("title")){
            if let Some(text) = node.text(){ self.title = text.to_string(); }else{ self.title = "".to_string(); }
        }else{
            self.title = "".to_string();
        }
        if let Some(node) = channel.children().find(|p| p.has_tag_name("link")){
            if let Some(text) = node.text(){ self.link = text.to_string(); }else{ self.link = "".to_string(); }
        }else{
            self.link = "".to_string();
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
            self.items.push(item);
        }
        self.items.reverse();
    }
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

