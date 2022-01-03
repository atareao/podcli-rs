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
    image: String,
    items: Vec<Item>,
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

    pub fn print(&self){
        println!("Title: {}", self.title);
        println!("Description: {}", self.description);
        println!("Enclosure: {}", self.enclosure);
        println!("Link: {}", self.link);
        println!("Image: {}", self.image);
    }
}

impl Podcast{
    pub fn new(url: &str)->Podcast{

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
}
