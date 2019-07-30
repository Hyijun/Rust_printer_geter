use reqwest;
use regex;
use regex::Regex;
use std::io::Read;
use std::fs::File;
use std::io;

fn main() {
    let rea:Regex = Regex::new("<li><a title=\"(?:.*?)\" href=\"(.*?)\" >(?:.*?)</a>(?:.*?)</li>").unwrap();
    let reb:Regex = Regex::new("<div class=\"cartoon_online_border\" >(?:.*?)<ul>(?:.*?)(.*?)</ul>").unwrap();
    let mut str_1 = String::new();
    str_1 = get_html("https://manhua.dmzj.com/biedangounijiangle/");
    let mut f = File::create("F:\\test\\a.html").unwrap();
    let mut str_2 = String::new();
    let mut urls:Vec<String>= Vec::new();
    str_1 = str_1.replace("\n", "");

    for each in reb.captures_iter(&str_1) {
        str_2 = each.get(0).unwrap().as_str().to_string();
    }

    for each in rea.captures_iter(&str_2){
        urls.push("https://manhua.dmzj.com/".to_string() + each.get(1).unwrap().as_str());
    }
//    println!("{:?}", urls);
}

fn get_html(url: &str) -> String{
    let mut html = reqwest::get(url).unwrap();
    let mut string = String::new();
    html.read_to_string(& mut string);
    string
}
