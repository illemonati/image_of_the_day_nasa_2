use rss::Channel;
use rss::{Enclosure, Item};
use std::io::{self, Cursor};
use reqwest;

// use getset;

#[derive(Debug)]
pub struct ImageOfTheDay {
    pub rss_feed: Channel,
    pub image: Cursor<Vec<u8>>,
    pub title: String,
    pub description: String,
    pub link: String,
}

impl ImageOfTheDay {
    pub fn new() -> ImageOfTheDay {
        let rss_url = "https://www.nasa.gov/rss/dyn/lg_image_of_the_day.rss";
        let channel = Channel::from_url(rss_url).expect("Channel Creation Error");
        let (image, title, description, link) = get_feed_inner(&channel);
        ImageOfTheDay {
            rss_feed: channel,
            image: image,
            title: title,
            description: description,
            link: link
        }
    }
}

fn get_feed_inner(channel: &Channel) -> (Cursor<Vec<u8>>, String, String, String){
    let items = channel.items();
    let first = &items[0];
    let title = first.title().expect("Error getting title").to_string();
    let description = first.description().expect("Error getting description").to_string();
    let link = first.link().expect("Error getting link").to_string();
    let enclosure = first.enclosure().expect("Error getting enclosure");
    let image_url = enclosure.url();

    let mut resp = reqwest::get(image_url).expect("request failed");
    let mut image = Cursor::new(Vec::new());
    io::copy(&mut resp, &mut image).expect("failed to copy content");

    (image, title, description, link)
}
