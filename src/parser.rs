use std::{
    collections::BTreeMap,
    fs::{self, DirEntry},
};

use markdown::{mdast::Node, Constructs, ParseOptions};

#[derive(Debug)]
pub struct FrontmatterData {
    pub title: String,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub date: String,
}

#[derive(Debug)]
pub struct Post {
    pub full_path: String,
    pub file_name: String,
    pub html: String,
    pub frontmatter: FrontmatterData,
}

const POSTS_FILE_PATH: &str = "/path/to/your/posts";

fn parse_html(post_markdown: &String) -> String {
    let html = markdown::to_html(post_markdown);
    return html;
}

fn parse_frontmatter_data(frontmatter_data: Node) -> Result<FrontmatterData, String> {
    match frontmatter_data {
        Node::Yaml(value) => {
            let parsed_ast: BTreeMap<String, String> = serde_yaml::from_str(&value.value).unwrap();
            let title_src = parsed_ast.get("title");
            let parsed_title;
            match title_src {
                Some(title) => {
                    parsed_title = title;
                }
                None => return Err("There was an error parsing the post title".to_owned()),
            }
            let date_src = parsed_ast.get("date");
            let parsed_date;
            match date_src {
                Some(date) => {
                    parsed_date = date;
                }
                None => return Err("There was an error parsing the post date".to_owned()),
            }
            let parsed_keywords = parsed_ast.get("keywords");
            let parsed_description = parsed_ast.get("description");

            Ok(FrontmatterData {
                title: parsed_title.to_string(),
                description: parsed_description.cloned(),
                keywords: parsed_keywords.cloned(),
                date: parsed_date.to_string(),
            })
        }
        _ => Err("there was an error parsing frontmatter data".to_owned()),
    }
}

fn parse_post(post_path: DirEntry) -> Result<Post, String> {
    let custom = Constructs {
        frontmatter: true,
        ..Constructs::gfm()
    };
    let parse_options = ParseOptions {
        constructs: custom,
        ..ParseOptions::default()
    };

    let full_path = post_path
        .path()
        .to_str()
        .unwrap_or("file_path error")
        .to_owned();
    let file_name = post_path
        .file_name()
        .to_str()
        .unwrap_or("file_name error")
        .to_owned();
    let post_markdown = fs::read_to_string(full_path.clone()).expect("should have read the file");

    let parsed_post_html = parse_html(&post_markdown);

    let parsed_ast_from_post = markdown::to_mdast(&post_markdown, &parse_options).unwrap();
    let frontmatter_data = parsed_ast_from_post
        .children()
        .unwrap()
        .clone()
        .into_iter()
        .nth(0)
        .unwrap();

    let post_frontmatter = parse_frontmatter_data(frontmatter_data);
    let parsed_post_frontmatter;
    match post_frontmatter {
        Ok(parsed_frontmatter_data) => {
            parsed_post_frontmatter = parsed_frontmatter_data;
        }
        Err(error_message) => {
            return Err(format!(
                "there was an error parsing frontmatter data {}",
                error_message
            ))
        }
    }

    let new_post = Post {
        full_path,
        file_name,
        html: parsed_post_html,
        frontmatter: parsed_post_frontmatter,
    };
    return Ok(new_post);
}

pub fn get_posts() -> Vec<Post> {
    let error_reading_files_message = format!("Error reading files at {}", POSTS_FILE_PATH);
    let post_paths = fs::read_dir(POSTS_FILE_PATH).expect(&error_reading_files_message);

    let mut parsed_posts = Vec::<Post>::new();

    for post_path in post_paths {
        match post_path {
            Ok(post_dir_entry) => {
                let parsed_post = parse_post(post_dir_entry);
                match parsed_post {
                    Ok(post) => {
                        parsed_posts.push(post);
                    }
                    Err(error_message) => {
                        println!("There was an error with the post {}", error_message);
                    }
                }
            }
            Err(error_message) => {
                println!("There was an error with the post: {}", error_message);
            }
        }
    }

    return parsed_posts;
}
