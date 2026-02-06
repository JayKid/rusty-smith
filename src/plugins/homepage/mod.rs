use std::fs;
use std::fs::File;
use std::io::Write;
use std::env;
use dotenv;
use crate::parser::Post;
use crate::plugin::{Plugin, Site};

// Template filepaths
const HOMEPAGE_TEMPLATE_FILE_PATH: &str = "./assets/templates/homepage.html";
const HOMEPAGE_POST_PARTIAL_FILE_PATH: &str = "./assets/templates/archive-item.html";

// Templates placeholders
const POST_ITEMS_PLACEHOLDER: &str = "{post_items}";
const HOST_PLACEHOLDER: &str = "{host}";
const WEBSITE_NAME: &str = "{website_name}";
const WEBSITE_LOGO_URL: &str = "{website_logo_url}";
const WEBSITE_DESCRIPTION: &str = "{website_description}";
const AUTHOR_NAME: &str = "{author_name}";

// Homepage Item partial
const POST_ITEM_LINK_PLACEHOLDER: &str = "{post_link}";
const POST_ITEM_DATE_TIMESTAMP_PLACEHOLDER: &str = "{post_date_timestamp}";
const POST_ITEM_DATE_READABLE_PLACEHOLDER: &str = "{post_date_human_readable}";
const POST_ITEM_TITLE_PLACEHOLDER: &str = "{post_title}";
const POST_ITEM_EXCERPT_PLACEHOLDER: &str = "{post_excerpt}";

pub struct HomepagePlugin;

impl HomepagePlugin {
    pub fn new() -> Self {
        HomepagePlugin
    }
}

impl Plugin for HomepagePlugin {
    fn name(&self) -> &str {
        "homepage"
    }

    fn run(&self, site: &mut Site) -> Result<(), Box<dyn std::error::Error>> {
        let binding = env::current_dir().unwrap().into_os_string();
        let current_path = binding.to_str().unwrap();
        let build_path = format!("{}/build/", current_path);
        let file_path = format!("{}index.html", build_path);

        // Get environment variables
        let website_name = dotenv::var("WEBSITE_NAME").expect("WEBSITE_NAME environment variable must be set");
        let author_name = dotenv::var("AUTHOR_NAME").expect("AUTHOR_NAME environment variable must be set");
        let host = dotenv::var("HOST").expect("HOST environment variable must be set");
        let website_logo_url = dotenv::var("WEBSITE_LOGO_URL").expect("WEBSITE_LOGO_URL environment variable must be set");
        let website_description = dotenv::var("WEBSITE_DESCRIPTION").expect("WEBSITE_DESCRIPTION environment variable must be set");

        // Read the post item template
        let post_item_template = fs::read_to_string(HOMEPAGE_POST_PARTIAL_FILE_PATH)?;
        let mut post_items = String::new();

        // Generate post items
        for post in &site.posts {
            // Format date for human readable display (YYYY/MM/DD)
            let date_human_readable = post.frontmatter.date.replace('-', "/");
            // Build full post URL
            let post_url = format!("{}/{}/", host, post.permalink);

            let post_item = post_item_template
                .replace(POST_ITEM_LINK_PLACEHOLDER, &post_url)
                .replace(POST_ITEM_DATE_TIMESTAMP_PLACEHOLDER, &post.frontmatter.date)
                .replace(POST_ITEM_DATE_READABLE_PLACEHOLDER, &date_human_readable)
                .replace(POST_ITEM_TITLE_PLACEHOLDER, &post.frontmatter.title)
                .replace(POST_ITEM_EXCERPT_PLACEHOLDER, &post.frontmatter.description.as_deref().unwrap_or(""));
            post_items.push_str(&post_item);
        }

        // Read the homepage template
        let mut homepage_template = fs::read_to_string(HOMEPAGE_TEMPLATE_FILE_PATH)?;

        // Replace placeholders in the homepage template
        homepage_template = homepage_template
            .replace(POST_ITEMS_PLACEHOLDER, &post_items)
            .replace(HOST_PLACEHOLDER, &host)
            .replace(WEBSITE_NAME, &website_name)
            .replace(WEBSITE_LOGO_URL, &website_logo_url)
            .replace(WEBSITE_DESCRIPTION, &website_description)
            .replace(AUTHOR_NAME, &author_name);

        // Write the final HTML
        let mut file = File::create(file_path)?;
        write!(file, "{}", homepage_template)?;
        Ok(())
    }
} 