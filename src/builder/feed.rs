use std::fs;
use std::time::SystemTime;

use chrono::{DateTime, Utc};

use crate::builder::{create_file, get_build_dir};
use crate::parser;


// Template filepaths
const FEED_TEMPLATE_FILE_PATH: &str = "./assets/templates/feed.xml";
const FEED_ENTRY_PARTIAL_FILE_PATH: &str = "./assets/templates/feed-entry.xml";

// Templates placeholders
// Feed
const WEBSITE_NAME_PLACEHOLDER: &str = "{website_name}";
const LAST_UPDATED_FEED_TIME_PLACEHOLDER: &str = "{last_updated_feed_time}";
const AUTHOR_NAME_PLACEHOLDER: &str = "{author_name}";
const HOST_PLACEHOLDER: &str = "{host}";
const ENTRIES_PLACEHOLDER: &str = "{entries}";

// Feed Entry partial
const TITLE_PLACEHOLDER: &str = "{title_placeholder}";
const LINK_PLACEHOLDER: &str = "{link_placeholder}";
const ID_PLACEHOLDER: &str = "{id_placeholder}";
const UPDATED_PLACEHOLDER: &str = "{updated_placeholder}";
const SUMMARY_PLACEHOLDER: &str = "{summary_placeholder}";

pub fn create_feed(posts: &Vec<parser::Post>) {
    let feed_template = get_feed_template();
    let feed_markup = feed_template
        .replace(ENTRIES_PLACEHOLDER, &get_feed_entries_contents(posts));

    let path = format!("{}/feed.xml", get_build_dir());

    create_file(&feed_markup, &path);
}

fn get_feed_template() -> String {
    let host: String = dotenv::var("HOST").expect("HOST environment variable must be set");
    let website_name: String = dotenv::var("WEBSITE_NAME").expect("WEBSITE_NAME environment variable must be set");
    let author_name: String = dotenv::var("AUTHOR_NAME").expect("AUTHOR_NAME environment variable must be set");
    
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    let last_updated_feed_time = now.to_rfc3339();

    let mut template_contents = fs::read_to_string(FEED_TEMPLATE_FILE_PATH).unwrap();
    template_contents = template_contents
        .replace(&WEBSITE_NAME_PLACEHOLDER, &website_name)
        .replace(&HOST_PLACEHOLDER, &host)
        .replace(&LAST_UPDATED_FEED_TIME_PLACEHOLDER, &last_updated_feed_time)
        .replace(&AUTHOR_NAME_PLACEHOLDER, &author_name);
    return template_contents;
}

fn get_feed_entries_contents(posts: &Vec<parser::Post>) -> String {
    let item_template = fs::read_to_string(FEED_ENTRY_PARTIAL_FILE_PATH).unwrap();
    let mut markup: String = "".to_owned();
    let host: String = dotenv::var("HOST").expect("HOST environment variable must be set");

    for post in posts {
        let post_title = &post.frontmatter.title;
        let post_link = &format!("{}/{}/", host, post.permalink);
        let post_id = &format!("{}/{}/", host, post.permalink);
        let post_updated: String = get_last_updated_time(&post.frontmatter.date);

        let mut post_description = "";
        if let Some(description) = &post.frontmatter.description {
            post_description = &description;
        }

        let item_markup = &item_template
            .replace(TITLE_PLACEHOLDER, &post_title)
            .replace(LINK_PLACEHOLDER, post_link)
            .replace(ID_PLACEHOLDER, post_id)
            .replace(UPDATED_PLACEHOLDER, &post_updated)
            .replace(SUMMARY_PLACEHOLDER, post_description);
        markup += item_markup;
    }
    return markup;
}

fn get_last_updated_time(date: &String) -> String {
    return format!("{}T00:00:00.000Z",date);
}
