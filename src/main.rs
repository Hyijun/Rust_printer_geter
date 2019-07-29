use reqwest;
use regex;
use regex::Regex;
use std::io::Read;
use std::fs::File;
use std::io;

fn main() {
    let rea:Regex = Regex::new("<li><a title=\"(?:.*?)\" href=\"(.*?)\" >(?:.*?)</a>(?:.*?)</li>").unwrap();
    let reb:Regex = Regex::new("<div class=\"cartoon_online_border\" >(?:.*?)<ul>(?:.*?)(.*?)</ul>").unwrap();
    let mut html = reqwest::get("https://manhua.dmzj.com/biedangounijiangle/").unwrap();
    let mut f = File::create("F:\\test\\a.html").unwrap();
    let mut str_1 = String::new();
    html.read_to_string(& mut str_1);
    str_1 = str_1.replace("\n", "");
//    println!("{}", str_1);
    for each in reb.captures_iter(&str_1) {
        println!("{}", each.get(0).unwrap().as_str());
    }
    println!("{:?}", reb.find(&str_1));
//    let mut buffer = [u8];
//    println!("{:?}", html.read(buffer));


}
