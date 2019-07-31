use reqwest;
use regex;
use regex::Regex;
use std::io::Read;
use std::fs::File;
use std::io;
use std::borrow::Borrow;
use std::fs;

fn main() {
//    let rea:Regex = Regex::new("<li><a title=\"(.*?)\" href=\"(.*?)\" >(?:.*?)</a>(?:.*?)</li>").unwrap();
//    let red:Regex = Regex::new("<li><a title=\"(.*?)\" href=\"(.*?)\"?class=\"color_red\">(?:.*?)</a>(?:.*?)</li>").unwrap();
    let rea:Regex = Regex::new("<li><a title=\"(.*?)\" href=\"(/biedangounijiangle/(\\d*?)\\.shtml)\" ").unwrap();
    let reb:Regex = Regex::new("<div class=\"cartoon_online_border\" >(?:.*?)<ul>(?:.*?)(.*?)</ul>").unwrap();
    let rec:Regex = Regex::new("var next_chapter_pages = '\\[\"b(.*?)/01\\.jpg").unwrap();
    let mut str_1 = String::new();
    let mut str_2 = String::new();
    let mut urls:Vec<String>= Vec::new();
    let mut pages:Vec<String> = Vec::new();
    let mut img_url = String::new();
    let mut dir:Vec<String> = Vec::new();
    let mut img_urls:Vec<&str> = Vec::new();
    str_1 = get_html("https://manhua.dmzj.com/biedangounijiangle/");
    str_1 = str_1.replace("\n", "");

    get_page_info(rea, &str_1, &mut dir, & mut urls);

    println!("{:?}", urls.len());
    for each in urls{
        pages.push(get_html(&each));

    }
    for ea in pages{
        for each in rec.captures_iter(ea.as_str()){
            img_url = "https://images.dmzj.com/b".to_string() + each.get(1).unwrap().as_str() + "/";
        }
    img_url = img_url.replace("\\", "");
    img_urls.push(img_url.as_str());
    }
    println!("{:?}", img_urls.len());
    let mut buffer:Vec<&str> = img_urls.clone();
    for each in img_urls{
        buffer.push(each);
    }
    img_urls = buffer;
    println!("{:?}", img_urls);

    for each in &dir{
        fs::create_dir("./".to_string() + each);
    }
    for ea in img_urls{
        for each in 1..99{
            let mut page = String::new();
            page = each.to_string();
            if page.len() == 1{
            page = "0".to_string() + page.as_str();
            }
            let mut f = File::create("./".to_string() + ea + "/" + page.as_str() + ".jpg").unwrap();
            let mut res = get_img(&(ea.to_string() + &page + ".jpg"));
            if res.status().is_success(){
                io::copy(& mut res, &mut f);
            }else {
                break;
            }
        }
    }
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

fn get_page_info(re: Regex, page_item: &str, dirs:&mut Vec<String>, urls:&mut Vec<String>){
    for each in re.captures_iter(page_item){
        dirs.push(each.get(1).unwrap().as_str().to_string());
        println!("{:?}", each);
        urls.push("https://manhua.dmzj.com/".to_string() + each.get(2).unwrap().as_str());
    }
}
