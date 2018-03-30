extern crate webdrill;
use webdrill::selenium::webdriver::remote::webdriver::WebDriver;
use webdrill::selenium::webdriver::common::by::By;

fn main() {
    let cli = WebDriver::new("http://localhost:4444/wd/hub").unwrap();
    cli.get("https://www.pixiv.net").unwrap();
    let element = cli.find_element(By::TAG_NAME, "form").unwrap();
    println!("{:?}", element);
}
