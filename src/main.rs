extern crate comrak;
extern crate easy_fs;
extern crate slime;
extern crate toml;
use easy_fs::{DirectoryInfo, FileInfo};
use slime::Slime;
use toml::Value as Toml;
use comrak::{markdown_to_html, ComrakOptions};

fn main() {
    let mut s = Slime::default();
    s.initialize().expect("failed to initialize Slime");

    let data_folder = DirectoryInfo::new("data/posts").unwrap();
    let mut posts_names = Vec::new();
    let mut posts_data = Vec::new();
    for file in data_folder.get_files().unwrap() {
        //println!("file: {:?}", &file);
        let ext = file.get_extension().unwrap();
        if ext == "toml" {
            posts_names.push(file.get_name_without_extension().unwrap());
        }
    }
    for post_name in &posts_names {
        println!("generating single post: {}", post_name);
        let mut data = s.load_toml_data(&format!("posts/{}", post_name))
            .expect("failed to load data");

        let content_file =
            FileInfo::new(&format!("data/posts/{}.md", post_name)).expect("no content file");
        let content = content_file
            .read_to_string()
            .expect("failed to read content");
        data["content"] = Toml::String(fix_html(&content));
        let data_cloned = data.clone();
        posts_data.push(data_cloned);
        s.generate("post", &post_name, &data)
            .expect("failed to generate");
    }
    let index_data = Toml::Array(posts_data);
    s.generate("index", "index", &index_data)
        .expect("failed to generate index");
}

fn fix_html(input: &str) -> String {
    let o = markdown_to_html(input, &ComrakOptions::default());
    let o = o.replace("<h1>", r#"<h1 class="title bigtext">"#);
    let o = o.replace("<p>", r#"<p class="text">"#);
    // let o = o.replace("<pre>", r#"<pre class="codeblock">"#);
    // let o = o.replace("<h2>", r#"<h2 class="subtitle">"#);
    //let o = o.replace("<codeblock>", r#"<code><pre>"#);
    //let o = o.replace("</codeblock>", r#"</pre></code>"#);
    // let o = input.replace("<code>", r#"<pre>"#);
    //let o = input.replace("</code>", r#"</pre>"#);
    o
    //input.to_string()
}
