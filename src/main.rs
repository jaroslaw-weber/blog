extern crate chrono;
extern crate comrak;
extern crate easy_fs;
extern crate slime;
extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate serde;
use comrak::{markdown_to_html, ComrakOptions};
use easy_fs::{DirectoryInfo, FileInfo};
use slime::Slime;
use toml::Value as Toml;

use chrono::DateTime;

#[derive(Deserialize, Serialize, Clone, Debug)]
struct PostData {
    title: String,
    url: String,
    time: String,
    content: Option<String>,
    image: String,
    image_contribution: String,
    intro: String,
    publish: Option<bool>,
}

fn main() {
    let mut s = Slime::default();
    s.initialize().expect("failed to initialize Slime");
    let posts_data: Vec<PostData> = get_sorted_posts_data(&s);
    generate_post_pages(&s, &posts_data);
    generate_index_page(&s, &posts_data);
    s.copy_static_files();
}

fn generate_post_pages(slime: &Slime, posts_data: &Vec<PostData>) {
    for data in posts_data {
        let post_name = &data.url;
        println!("post name: {:?}", &data.url);
        println!("post title: {:?}", &data.url);
        //println!("post data: {:?}", &data);
        slime
            .generate("post", &post_name, &data)
            .expect("failed to generate");
    }
}

fn generate_index_page(slime: &Slime, posts_data: &Vec<PostData>) {
    let toml_posts: Vec<Toml> = posts_data
        .into_iter()
        .map(|x| {
            let as_string = toml::to_string(&x).expect("failed converting");
            toml::from_str(&as_string).expect("failed converting")
        })
        .collect();
    let index_data = Toml::Array(toml_posts);
    //println!("data: {:?}", index_data);
    slime
        .generate("index", "index", &index_data)
        .expect("failed to generate index");
}

fn get_sorted_posts_data(slime: &Slime) -> Vec<PostData> {
    let data_folder = DirectoryInfo::new("data/posts").unwrap();
    let posts_names = get_posts_names(&data_folder);
    let mut posts_data = Vec::new();

    for post_name in &posts_names {
        println!("parsing metadata: {}", post_name);
        let mut data = get_post_data(&slime, &post_name);
        if !data.should_publish()
        {
            //dont add to list if "publish" value is false
            println!("not publishing: {}", &post_name);
            continue;
        }
        let content = get_post_content(&post_name);
        data.content = Some(fix_html(&content));
        posts_data.push(data);
    }

    posts_data.sort_by_key(|ref data| data.get_datetime().expect("failed to get datetime"));
    posts_data.reverse();
    posts_data
}

fn get_post_data(slime: &Slime, post_name: &str) -> PostData {
    slime
        .load_toml_data(&format!("posts/{}", post_name))
        .expect("failed to load data")
        .try_into()
        .expect("failed to convert data into struct")
}

fn get_post_content(post_name: &str) -> String {
    let content_file =
        FileInfo::new(&format!("data/posts/{}.md", post_name)).expect("no content file");
    let content = content_file
        .read_to_string()
        .expect("failed to read content");
    content
}

fn get_posts_names(data_folder: &DirectoryInfo) -> Vec<String> {
    let mut posts_names = Vec::new();
    for file in data_folder
        .get_files()
        .expect("failed to get files in posts folder")
    {
        let ext_option = file.get_extension();
        match ext_option {
            Some(ext) => if ext == "toml" {
                posts_names.push(file.get_name_without_extension().unwrap());
            },
            None => {}
        }
    }
    posts_names
}

fn fix_html(input: &str) -> String {
    let o = markdown_to_html(input, &ComrakOptions::default());
    let o = o.replace("<h1>", r#"<h1 class="title bigtext primary-text">"#);
    let o = o.replace("<p>", r#"<p class="text">"#);
    o
}

impl PostData {
    fn should_publish(&self) -> bool {
        self.publish.unwrap_or(true)
    }

    fn get_datetime(&self) -> Result<DateTime<chrono::FixedOffset>, chrono::ParseError> {
        let input = &self.time;
        //DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z")
        //2017-13-31"
        let add = "T00:00:00+00:00"; //todo use rfc3339
        let mut rfc_compatibile_time = input.to_string();
        rfc_compatibile_time.push_str(add);
        println!("rfc:{}", rfc_compatibile_time);
        DateTime::parse_from_rfc3339(&rfc_compatibile_time)
    }
}
