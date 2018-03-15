extern crate chrono;
extern crate comrak;
extern crate easy_fs;
extern crate slime;
extern crate toml;
use easy_fs::{DirectoryInfo, FileInfo};
use slime::Slime;
use toml::Value as Toml;
use comrak::{markdown_to_html, ComrakOptions};

use chrono::DateTime;

fn main() {
    let mut s = Slime::default();
    s.initialize().expect("failed to initialize Slime");

    let data_folder = DirectoryInfo::new("data/posts").unwrap();
    let mut posts_names = Vec::new();
    let mut posts_data = Vec::new();
    for file in data_folder
        .get_files()
        .expect("failed to get files in posts folder")
    {
        //println!("file: {:?}", &file);

        let ext_option = file.get_extension();
        match ext_option {
            Some(ext) => {
                if ext == "toml" {
                    posts_names.push(file.get_name_without_extension().unwrap());
                }
            }
            None => {}
        }
    }
    for post_name in &posts_names {
        println!("generating single post: {}", post_name);
        let mut data = s.load_toml_data(&format!("posts/{}", post_name))
            .expect("failed to load data");

        let dt = (&data["time"]).as_str().expect("failed to get time").to_string();
        let dt_parsed = parse_date(&dt).expect("failed to parse data");
        println!("dt: {} , {:?}", dt, &dt_parsed);

        let content_file =
            FileInfo::new(&format!("data/posts/{}.md", post_name)).expect("no content file");
        let content = content_file
            .read_to_string()
            .expect("failed to read content");
        data["content"] = Toml::String(fix_html(&content));
        let data_cloned = data.clone();
        posts_data.push((data_cloned, dt_parsed));
        s.generate("post", &post_name, &data)
            .expect("failed to generate");
    }
    posts_data.sort_by_key(|&(ref _data, datetime)| datetime);
    posts_data.reverse();
    let mut posts_data_sorted = Vec::new();
    for (data, _datetime) in posts_data
    {
        posts_data_sorted.push(data);
    }
    let index_data = Toml::Array(posts_data_sorted);
    s.generate("index", "index", &index_data)
        .expect("failed to generate index");
}

fn parse_date(input: &str) -> Result<DateTime<chrono::FixedOffset>, chrono::ParseError> {
    //DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z")
    //2017-13-31"
    let add = "T00:00:00+00:00";//to use rfc3339
    let mut rfc_compatibile_time = input.to_string();
    rfc_compatibile_time.push_str(add);
    println!("rfc:{}", rfc_compatibile_time);
    DateTime::parse_from_rfc3339(&rfc_compatibile_time)
}



fn fix_html(input: &str) -> String {
    let o = markdown_to_html(input, &ComrakOptions::default());
    let o = o.replace("<h1>", r#"<h1 class="title bigtext primary-text">"#);
    let o = o.replace("<p>", r#"<p class="text">"#);
    // let o = o.replace("<pre>", r#"<pre class="codeblock">"#);
    // let o = o.replace("<h2>", r#"<h2 class="subtitle">"#);
    //let o = o.replace("<codeblock>", r#"<code><pre>"#);
    //let o = o.replace("</codeblock>", r#"</pre></code>"#);
    // let o = o.replace("<code>", r#"<code class="inline-code">"#);
    //let o = input.replace("</code>", r#"</pre>"#);
    o
    //input.to_string()
}
