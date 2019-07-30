use reqwest;
use regex;
use regex::Regex;
use std::io::Read;
use std::fs::File;
use std::io;
use std::borrow::Borrow;
use std::fs;

fn main() {
    let rea:Regex = Regex::new("<li><a title=\"(.*?)\" href=\"(.*?)\" >(?:.*?)</a>(?:.*?)</li>").unwrap();
    let reb:Regex = Regex::new("<div class=\"cartoon_online_border\" >(?:.*?)<ul>(?:.*?)(.*?)</ul>").unwrap();
    let rec:Regex = Regex::new("var next_chapter_pages = '\\[\"b(.*?)/01\\.jpg").unwrap();
    let mut str_1 = String::new();
    let mut str_2 = String::new();
    let mut urls:Vec<String>= Vec::new();
    let mut pages:Vec<String> = Vec::new();
    let mut img_url = String::new();
    let mut dir:Vec<&str> = Vec::new();
    let mut img_urls:Vec<String> = Vec::new();
    str_1 = get_html("https://manhua.dmzj.com/biedangounijiangle/");
    str_1 = str_1.replace("\n", "");

    for each in reb.captures_iter(&str_1) {
        str_2 = each.get(1).unwrap().as_str().to_string();
    }

    for each in rea.captures_iter(&str_2){
        dir.push(each.get(1).unwrap().as_str());
        urls.push("https://manhua.dmzj.com/".to_string() + each.get(2).unwrap().as_str());
    }
    for each in urls{
        pages.push(get_html(&each));
    }
    for ea in &pages{
        for each in rec.captures_iter(ea){
            img_url = "https://images.dmzj.com/b".to_string() + each.get(1).unwrap().as_str() + "/";
            img_url = img_url.replace("\\", "");
        }
        img_urls.push(img_url.clone());
    }
    let mut buffer:Vec<String> = img_urls.clone();
    for each in img_urls{
        buffer.push(each.to_string());
    }
    img_urls = buffer;
    println!("{:?}", img_urls);

    let mut i:usize = 0;
    for ea in &dir{
        fs::create_dir("./".to_string() + ea);
        let _img_url = &img_urls[i];
        for each in 1..99{
            let mut page = String::new();
            page = each.to_string();
            if page.len() == 1{
            page = "0".to_string() + page.as_str();
            }
            let mut f = File::create("./".to_string() + ea + "/" + page.as_str() + ".jpg").unwrap();
            let mut res = get_img(&(_img_url.clone() + &page + ".jpg"));
            if res.status().is_success(){
                io::copy(& mut res, &mut f);
            }else {
                break;
            }
        }
        i += 1;
}
//    println!("{:?}", urls);
}

fn get_html(url: &str) -> String{
    let mut html = reqwest::get(url).unwrap();
    let mut string = String::new();
    html.read_to_string(& mut string);
    string
}


fn get_img(url: &str) -> reqwest::Response{
    println!("{}", url);
    reqwest::get(url).unwrap()
}
