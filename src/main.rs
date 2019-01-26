extern crate web_view;
extern crate rss;
extern crate reqwest;
extern crate base64;
// #[macro_use]
// extern crate getset;
mod interface;
mod imotd;

fn main() {
    interface::run_gui();
    // test_rss();
}

fn test_rss() {
    let image_of_the_day = imotd::ImageOfTheDay::new();
    println!("{:?}", image_of_the_day);
}
