

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use web_view::*;
use crate::imotd::ImageOfTheDay;
use base64::{encode, decode};

pub fn run_gui() {
    // let title = Arc::new(Mutex::new(String::from("Press button!")));
    // let title_inner = title.clone();
    let webview = web_view::builder()
        .title("Nasa Image Of The Day")
        .content(Content::Html(HTML))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(0)
        .invoke_handler(|webview, arg| {
            match arg {
                "get_feed" => {
                    *webview.user_data_mut() += 10;
                    let image_of_the_day = ImageOfTheDay::new();
                    // let mut title = title.lock().unwrap();
                    let title = image_of_the_day.title;
                    let description = image_of_the_day.description;
                    let link = image_of_the_day.link;
                    let image = image_of_the_day.image;
                    let image = format!("data:image/jpg;base64, {}", encode(&image.into_inner()));
                    render(webview, title, description, link, image)?;
                }
                "exit" => {
                    webview.terminate();
                }
                _ => unimplemented!(),
            };
            Ok(())
        })
        .build()
        .unwrap();

    let handle = webview.handle();
    webview.run().unwrap();
}

fn render(webview: &mut WebView<i32>, title: String, description: String, link: String, image: String) -> WVResult {
    let user_data = *webview.user_data();
    // println!("Title: {}, userdata: {}", title, user_data);
    webview.eval(format!("updateFeed(\"{}\", \"{}\",\"{}\", \"{}\")", title, description, link, image).as_str())
}

const HTML: &str = include_str!("interface.html");
