use std::fs;
use dotenv;

use crate::builder::{create_file, get_build_dir};
use crate::parser;

// Template filepaths
const POST_TEMPLATE_FILE_PATH: &str = "./assets/templates/post.html";

// Templates placeholders
const HOST_PLACEHOLDER: &str = "{host}";
const WEBSITE_NAME: &str = "{website_name}";
const AUTHOR_NAME: &str = "{author_name}";
const POST_ITEM_DATE_TIMESTAMP_PLACEHOLDER: &str = "{post_date_timestamp}";
const POST_ITEM_DATE_READABLE_PLACEHOLDER: &str = "{post_date_human_readable}";
const POST_ITEM_TITLE_PLACEHOLDER: &str = "{post_title}";
const POST_ITEM_DESCRIPTION_PLACEHOLDER: &str = "{post_description}";
const POST_ITEM_CONTENT_PLACEHOLDER: &str = "{post_content}";
const POST_ITEM_URL_PLACEHOLDER: &str = "{post_url}";
const POST_ITEM_KEYWORDS_PLACEHOLDER: &str = "{post_keywords}";
const POST_ITEM_IMAGE_URL_PLACEHOLDER: &str = "{post_image_url}";

fn wrap_post_with_layout(post: &parser::Post) -> String {
    let host: String = dotenv::var("HOST").expect("HOST environment variable must be set");
    let website_name: String = dotenv::var("WEBSITE_NAME").expect("WEBSITE_NAME environment variable must be set");
    let author_name: String = dotenv::var("AUTHOR_NAME").expect("AUTHOR_NAME environment variable must be set");
    let image = &format!("{}/img/logo.png", host); // TO-DO: Implement support for image in frontmatter parser
    let post_link = &format!("{}/{}/", host, post.full_path);
    let mut post_description = "";
    if let Some(description) = &post.frontmatter.description {
        post_description = &description;
    }

    let mut post_keywords = "";
    if let Some(keywords) = &post.frontmatter.keywords {
        post_keywords = &keywords;
    }

    let post_template = fs::read_to_string(POST_TEMPLATE_FILE_PATH).unwrap();

    let post_markup = post_template
        .replace(HOST_PLACEHOLDER, &host)
        .replace(WEBSITE_NAME, &website_name)
        .replace(AUTHOR_NAME, &author_name)
        .replace(POST_ITEM_DATE_TIMESTAMP_PLACEHOLDER, &post.frontmatter.date)
        .replace(
            POST_ITEM_DATE_READABLE_PLACEHOLDER,
            &post.frontmatter.date.replace('-', "/"),
        )
        .replace(POST_ITEM_TITLE_PLACEHOLDER, &post.frontmatter.title)
        .replace(POST_ITEM_DESCRIPTION_PLACEHOLDER, post_description)
        .replace(POST_ITEM_CONTENT_PLACEHOLDER, &post.html)
        .replace(POST_ITEM_URL_PLACEHOLDER, post_link)
        .replace(POST_ITEM_KEYWORDS_PLACEHOLDER, post_keywords)
        .replace(POST_ITEM_IMAGE_URL_PLACEHOLDER, image);

    return post_markup;
}

fn get_post_full_path(post: &parser::Post) -> String {
    let build_path = get_build_dir();
    let full_dir_path = format!("{}/{}", build_path, post.permalink);
    return full_dir_path;
}

fn create_post_file(post: &parser::Post) -> () {
    let full_path = get_post_full_path(&post);
    let file_path = format!("{}/index.html", full_path);

    let wrapped_post_contents = wrap_post_with_layout(&post);
    create_file(&wrapped_post_contents, &file_path)
}

pub fn create_post_dir_and_file(post: &parser::Post) -> () {
    let full_dir_path = get_post_full_path(&post);

    match fs::create_dir(full_dir_path) {
        Ok(_) => {
            // println!("created dir succesfully");
        }
        Err(msg) => {
            println!("failed to create directory, see error: {}", msg);
        }
    }

    create_post_file(post);
}
