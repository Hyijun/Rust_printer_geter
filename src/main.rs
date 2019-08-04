use reqwest;
use regex;
use regex::Regex;
use std::io::Read;
use std::fs::File;
use std::io;
use std::borrow::Borrow;
use std::fs;

#[derive(Clone, Debug, Eq, PartialEq)]
// In fact, I can't know what's the meaning of those words.
enum UrlType{
    Normal,
    Pic,
    None,
}

#[derive(Clone, Debug)]
struct ImgInfo{
    url: String,
    img_type:UrlType
}
impl ImgInfo{
    fn new(u:String, type_:UrlType) ->ImgInfo{
        ImgInfo{
            url:u,
            img_type:type_
        }
    }
}

fn main() {
//    let rea:Regex = Regex::new("<li><a title=\"(.*?)\" href=\"(.*?)\" >(?:.*?)</a>(?:.*?)</li>").unwrap();
//    let red:Regex = Regex::new("<li><a title=\"(.*?)\" href=\"(.*?)\"?class=\"color_red\">(?:.*?)</a>(?:.*?)</li>").unwrap();
//    let rec: Regex = Regex::new("var next_chapter_pages = '\\[\"b(.*?)/(01|pic_001)|(.*?_p0_master1200)?\\.jpg").unwrap();
    let rea: Regex = Regex::new("<li><a title=\"(.*?)\" href=\"(/biedangounijiangle/(\\d*?)\\.shtml)\"").unwrap();
    let reb: Regex = Regex::new("<div class=\"cartoon_online_border\" >(?:.*?)<ul>(?:.*?)(.*?)</ul>").unwrap();
    let rec: Regex = Regex::new("var next_chapter_pages = \'\\[\"b(.*?)/(01|pic_001)\\.jpg").unwrap();
//    let red: Regex = Regex::new("var next_chapter_pages = '\\[\"b(.*?)/(01|pic_001)\\.jpg").unwrap();
    let mut str_1 = String::new();
    let mut str_2 = String::new();
    let mut urls: Vec<String> = Vec::new();
    let mut pages: Vec<String> = Vec::new();
    let mut dir: Vec<String> = Vec::new();
    let mut img_urls: Vec<ImgInfo> = Vec::new();
    str_1 = get_html("https://manhua.dmzj.com/biedangounijiangle/");
    str_1 = str_1.replace("\n", "");

    get_page_info(rea, &str_1, &mut dir, &mut urls);

    let t = 0;
    for each in urls {
        pages.push(get_html(&each));
    }
    for ea in pages {
        print!("{}：", ea);
        println!("{}", rec.is_match(&ea));
        if !rec.is_match(&ea) {
            img_urls.push(ImgInfo::new("".to_string(), UrlType::None));
            println!("skip:{:?}", ea);
        }
        for each in rec.captures_iter(ea.as_str()) {
            if each.get(2).unwrap().as_str() == "01" {
                img_urls.push(ImgInfo::new(("https://images.dmzj.com/b".to_string() + each.get(1).unwrap().as_str() + "/").replace("\\", ""), UrlType::Normal));
            } else {
                img_urls.push(ImgInfo::new(("https://images.dmzj.com/b".to_string() + each.get(1).unwrap().as_str() + "/").replace("\\", ""), UrlType::Pic));
            }
        }
    }

    for each in &dir {
        fs::create_dir("./".to_string() + each);
    }
    let mut index: usize = 0;
    for ea in img_urls {
        for each in 1..99 {
            if ea.img_type == UrlType::None {
                break;
            }
            let mut page = String::new();
            page = each.to_string();
            if page.len() == 1 {
                page = "0".to_string() + page.as_str();
            }
            let file_path = "./".to_string() + &dir[index] + "/" + page.as_str() + ".jpg";
            if ea.img_type == UrlType::Pic {
                let mut res = get_img(&(ea.url.to_string() + "pic_0" + &page + ".jpg"));
                if !write_to_file(&mut res, file_path) { break }
            } else {
                let mut res = get_img(&(ea.url.to_string() + &page + ".jpg"));
                if !write_to_file(&mut res, file_path) { break }
            }
        }
        index += 1;
        if index == dir.len() {
            break;
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
    println!("Getting:{}", url);
    reqwest::get(url).unwrap()
}

fn get_page_info(re: Regex, page_item: &str, dirs:&mut Vec<String>, urls:&mut Vec<String>){
    for each in re.captures_iter(page_item) {
        dirs.push(each.get(1).unwrap().as_str().to_string());
        urls.push("https://manhua.dmzj.com/".to_string() + each.get(2).unwrap().as_str());
    }
}

fn write_to_file(reader: & mut reqwest::Response, path:String) ->bool {
    if reader.status().is_success(){
        let mut f = File::create(path).unwrap();
        io::copy(reader, & mut f);
        true
    }else {
        false
    }
}
