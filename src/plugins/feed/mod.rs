use std::fs::{self, File};
use std::io::Write;
use std::env;
use dotenv;
use crate::parser::Post;
use crate::plugin::{Plugin, Site};

// Template filepaths
const FEED_TEMPLATE_FILE_PATH: &str = "./assets/templates/feed.xml";

// Templates placeholders
const HOST_PLACEHOLDER: &str = "{host}";
const WEBSITE_NAME: &str = "{website_name}";
const WEBSITE_DESCRIPTION: &str = "{website_description}";
const POST_ITEM_DATE_TIMESTAMP_PLACEHOLDER: &str = "{post_date_timestamp}";
const POST_ITEM_TITLE_PLACEHOLDER: &str = "{post_title}";
const POST_ITEM_DESCRIPTION_PLACEHOLDER: &str = "{post_description}";
const POST_ITEM_URL_PLACEHOLDER: &str = "{post_url}";

pub struct FeedPlugin;

impl FeedPlugin {
    pub fn new() -> Self {
        FeedPlugin
    }
}

impl Plugin for FeedPlugin {
    fn name(&self) -> &str {
        "feed"
    }

    fn run(&self, site: &mut Site) -> Result<(), Box<dyn std::error::Error>> {
        // Get environment variables
        let website_name = dotenv::var("WEBSITE_NAME").expect("WEBSITE_NAME environment variable must be set");
        let website_description = dotenv::var("WEBSITE_DESCRIPTION").expect("WEBSITE_DESCRIPTION environment variable must be set");
        let host = dotenv::var("HOST").expect("HOST environment variable must be set");

        // Read the feed template
        let feed_template = fs::read_to_string(FEED_TEMPLATE_FILE_PATH)?;

        // Generate feed items
        let mut feed_items = String::new();
        for post in &site.posts {
            feed_items.push_str(&format!(
                "    <item>
        <title>{}</title>
        <link>{}{}</link>
        <description>{}</description>
        <pubDate>{}</pubDate>
        <guid>{}{}</guid>
    </item>\n",
                post.frontmatter.title,
                host,
                post.permalink,
                post.frontmatter.description.as_deref().unwrap_or(""),
                post.frontmatter.date,
                host,
                post.permalink
            ));
        }

        // Replace placeholders in the template
        let feed_xml = feed_template
            .replace(HOST_PLACEHOLDER, &host)
            .replace(WEBSITE_NAME, &website_name)
            .replace(WEBSITE_DESCRIPTION, &website_description)
            .replace("{feed_items}", &feed_items);

        // Write the feed file
        let binding = env::current_dir().unwrap().into_os_string();
        let current_path = binding.to_str().unwrap();
        let build_path = format!("{}/build/", current_path);
        let feed_path = format!("{}feed.xml", build_path);
        let mut file = File::create(feed_path)?;
        write!(file, "{}", feed_xml)?;

        Ok(())
    }
} 