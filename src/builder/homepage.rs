use std::fs;
use dotenv;

use crate::builder::{create_file, get_build_dir};
use crate::parser;

// Template filepaths
const HOMEPAGE_TEMPLATE_FILE_PATH: &str = "./assets/templates/homepage.html";
const HOMEPAGE_POST_PARTIAL_FILE_PATH: &str = "./assets/templates/archive-item.html";

// Templates placeholders
// Homepage
const POST_ITEMS_PLACEHOLDER: &str = "{post_items}";
const HOST_PLACEHOLDER: &str = "{host}";

// Homepage Item partial
const POST_ITEM_LINK_PLACEHOLDER: &str = "{post_link}";
const POST_ITEM_DATE_TIMESTAMP_PLACEHOLDER: &str = "{post_date_timestamp}";
const POST_ITEM_DATE_READABLE_PLACEHOLDER: &str = "{post_date_human_readable}";
const POST_ITEM_TITLE_PLACEHOLDER: &str = "{post_title}";
const POST_ITEM_EXCERPT_PLACEHOLDER: &str = "{post_excerpt}";

pub fn create_homepage(posts: &Vec<parser::Post>) -> () {
    let mut homepage_markup = get_homepage_markup(posts);
    homepage_markup = homepage_markup.replace(&POST_ITEMS_PLACEHOLDER, &homepage_markup);

    let path = format!("{}/index.html", get_build_dir());

    create_file(&homepage_markup, &path);
}

fn get_posts_markup(posts: &Vec<parser::Post>) -> String {
    let item_template = fs::read_to_string(HOMEPAGE_POST_PARTIAL_FILE_PATH).unwrap();
    let mut markup: String = "".to_owned();
    let host: String = dotenv::var("HOST").expect("HOST environment variable must be set");

    for post in posts {
        let post_link = &format!("{}/{}/", host, post.permalink);
        let post_date = &post.frontmatter.date;

        let mut post_description = "";
        if let Some(description) = &post.frontmatter.description {
            post_description = &description;
        }

        let item_markup = &item_template
            .replace(POST_ITEM_LINK_PLACEHOLDER, &post_link)
            .replace(POST_ITEM_DATE_TIMESTAMP_PLACEHOLDER, post_date)
            .replace(POST_ITEM_DATE_READABLE_PLACEHOLDER, post_date)
            .replace(POST_ITEM_TITLE_PLACEHOLDER, &post.frontmatter.title)
            .replace(POST_ITEM_EXCERPT_PLACEHOLDER, post_description);
        markup += item_markup;
    }
    return markup;
}

fn get_home_template() -> String {
    let host: String = dotenv::var("HOST").expect("HOST environment variable must be set");
    let mut template_contents = fs::read_to_string(HOMEPAGE_TEMPLATE_FILE_PATH).unwrap();
    template_contents = template_contents.replace(&HOST_PLACEHOLDER, &host);
    return template_contents;
}

fn get_homepage_markup(posts: &Vec<parser::Post>) -> String {
    let template = get_home_template();
    let posts_markup = get_posts_markup(posts);
    return template.replace(&POST_ITEMS_PLACEHOLDER, &posts_markup);
}
