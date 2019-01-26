
extern crate web_view;
extern crate rss;
extern crate reqwest;
extern crate base64;
mod interface;
mod imotd;

#[no_mangle]
pub fn run() {
    interface::run_gui();
}

#[no_mangle]
pub fn test_rss() {
    let image_of_the_day = imotd::ImageOfTheDay::new();
    println!("{:?}", image_of_the_day);
}
