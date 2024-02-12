use std::{
    collections::BTreeMap,
    fs::{self, DirEntry},
};

use markdown::{mdast::Node, CompileOptions, Constructs, Options, ParseOptions};

#[derive(Debug)]
pub struct FrontmatterData {
    pub date: String,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub title: String,
}

#[derive(Debug)]
pub struct Post {
    pub file_name: String,
    pub frontmatter: FrontmatterData,
    pub full_path: String,
    pub html: String,
    pub permalink: String,
}

const POSTS_FILE_PATH: &str = "/home/jay/code/rusty-smith/posts";

fn get_permalink_from_title(post_title: &String) -> String {
    let lowercase = post_title.to_lowercase();
    return str::replace(&lowercase, " ", "-");
}

fn parse_html(post_markdown: &String) -> String {
    let parse_options = Options {
        compile: CompileOptions {
            allow_dangerous_html: true, // I need it for my mixed Markdown + HTML post style
            ..CompileOptions::default()
        },
        parse: ParseOptions {
            constructs: Constructs {
                frontmatter: true,
                ..Constructs::default()
            },
            ..ParseOptions::default()
        },
    };
    let html = markdown::to_html_with_options(post_markdown, &parse_options).unwrap();
    return html;
}

fn parse_frontmatter_data(frontmatter_data: Node) -> Result<FrontmatterData, String> {
    match frontmatter_data {
        Node::Yaml(value) => {
            let parsed_ast: BTreeMap<String, String> = serde_yaml::from_str(&value.value).unwrap();

            let title_src = parsed_ast.get("title");
            let parsed_title;
            if let Some(title_value) = title_src {
                parsed_title = title_value;
            } else {
                return Err("There was an error parsing the post title".to_owned());
            }

            let date_src = parsed_ast.get("date");
            let parsed_date;
            if let Some(date_value) = date_src {
                parsed_date = date_value;
            } else {
                return Err("There was an error parsing the post date".to_owned());
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

    let full_path = &post_path
        .path()
        .to_str()
        .unwrap_or("file_path error")
        .to_owned();
    let file_name = post_path
        .file_name()
        .to_str()
        .unwrap_or("file_name error")
        .to_owned();
    let post_markdown =
        fs::read_to_string(String::from(full_path)).expect("should have read the file");

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
    let permalink = get_permalink_from_title(&parsed_post_frontmatter.title);

    let new_post = Post {
        file_name,
        frontmatter: parsed_post_frontmatter,
        full_path: String::from(full_path),
        html: parsed_post_html,
        permalink: permalink,
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
