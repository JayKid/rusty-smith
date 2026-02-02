use std::fs::{self, File};
use std::io::Write;
use std::env;
use dotenv;
use crate::parser::Post;
use crate::plugin::{Plugin, Site};

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
const THEME_CLASS_PLACEHOLDER: &str = "{theme_class}";

pub struct PostPlugin;

impl PostPlugin {
    pub fn new() -> Self {
        PostPlugin
    }
}

impl Plugin for PostPlugin {
    fn name(&self) -> &str {
        "post"
    }

    fn run(&self, site: &mut Site) -> Result<(), Box<dyn std::error::Error>> {
        // Get environment variables
        let website_name = dotenv::var("WEBSITE_NAME").expect("WEBSITE_NAME environment variable must be set");
        let author_name = dotenv::var("AUTHOR_NAME").expect("AUTHOR_NAME environment variable must be set");
        let host = dotenv::var("HOST").expect("HOST environment variable must be set");

        // Read the post template
        let post_template = fs::read_to_string(POST_TEMPLATE_FILE_PATH)?;

        for post in &site.posts {
            let binding = env::current_dir().unwrap().into_os_string();
            let current_path = binding.to_str().unwrap();
            let build_path = format!("{}/build/", current_path);
            let post_dir_path = format!("{}{}", build_path, post.permalink);
            let file_path = format!("{}/index.html", post_dir_path);

            fs::create_dir_all(&post_dir_path)?;
            let mut file = File::create(file_path)?;

            // Replace placeholders in the template
            let post_html = post_template
                .replace(HOST_PLACEHOLDER, &host)
                .replace(WEBSITE_NAME, &website_name)
                .replace(AUTHOR_NAME, &author_name)
                .replace(POST_ITEM_DATE_TIMESTAMP_PLACEHOLDER, &post.frontmatter.date)
                .replace(POST_ITEM_DATE_READABLE_PLACEHOLDER, &post.frontmatter.date)
                .replace(POST_ITEM_TITLE_PLACEHOLDER, &post.frontmatter.title)
                .replace(POST_ITEM_DESCRIPTION_PLACEHOLDER, &post.frontmatter.description.as_deref().unwrap_or(""))
                .replace(POST_ITEM_CONTENT_PLACEHOLDER, &post.html)
                .replace(POST_ITEM_URL_PLACEHOLDER, &post.permalink)
                .replace(POST_ITEM_KEYWORDS_PLACEHOLDER, &post.frontmatter.keywords.as_deref().unwrap_or(""))
                .replace(POST_ITEM_IMAGE_URL_PLACEHOLDER, &format!("{}/img/logo.png", host))
                .replace(THEME_CLASS_PLACEHOLDER, post.frontmatter.theme_class());

            write!(file, "{}", post_html)?;
        }
        Ok(())
    }
} 