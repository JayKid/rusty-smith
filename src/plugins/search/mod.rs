use std::env;
use std::fs::{self, File};
use std::io::Write;

use dotenv;

use crate::plugin::{Plugin, Site};

const SEARCH_TEMPLATE_FILE_PATH: &str = "./assets/templates/search.html";

// Template placeholders
const HOST_PLACEHOLDER: &str = "{host}";
const WEBSITE_NAME_PLACEHOLDER: &str = "{website_name}";
const WEBSITE_DESCRIPTION_PLACEHOLDER: &str = "{website_description}";
const WEBSITE_LOGO_URL_PLACEHOLDER: &str = "{website_logo_url}";
const AUTHOR_NAME_PLACEHOLDER: &str = "{author_name}";
const TWITTER_HANDLE_PLACEHOLDER: &str = "{twitter_handle}";
const RESOURCES_PLACEHOLDER: &str = "{resources}";

pub struct SearchPlugin;

impl SearchPlugin {
    pub fn new() -> Self {
        SearchPlugin
    }

    fn format_date(date: &str) -> String {
        // Input: "YYYY-MM-DD", Output: "YYYY/MM/DD"
        date.replace("-", "/")
    }

    fn generate_search_json(site: &Site, host: &str) -> String {
        let items: Vec<String> = site
            .posts
            .iter()
            .map(|post| {
                let url = format!("{}/{}/", host, post.permalink);
                let date_timestamp = &post.frontmatter.date;
                let date_human_readable = Self::format_date(date_timestamp);
                let excerpt = post
                    .frontmatter
                    .description
                    .as_deref()
                    .unwrap_or("")
                    .replace("\"", "\\\"");
                let title = post.frontmatter.title.replace("\"", "\\\"");

                format!(
                    r#"{{"title":"{}","url":"{}","dateTimestamp":"{}","dateHumanReadable":"{}","excerpt":"{}"}}"#,
                    title, url, date_timestamp, date_human_readable, excerpt
                )
            })
            .collect();

        format!("[{}]", items.join(","))
    }
}

impl Plugin for SearchPlugin {
    fn name(&self) -> &str {
        "search"
    }

    fn run(&self, site: &mut Site) -> Result<(), Box<dyn std::error::Error>> {
        // Get environment variables
        let host = dotenv::var("HOST").expect("HOST environment variable must be set");
        let website_name =
            dotenv::var("WEBSITE_NAME").expect("WEBSITE_NAME environment variable must be set");
        let website_description = dotenv::var("WEBSITE_DESCRIPTION")
            .expect("WEBSITE_DESCRIPTION environment variable must be set");
        let website_logo_url = dotenv::var("WEBSITE_LOGO_URL")
            .expect("WEBSITE_LOGO_URL environment variable must be set");
        let author_name =
            dotenv::var("AUTHOR_NAME").expect("AUTHOR_NAME environment variable must be set");
        let twitter_handle =
            dotenv::var("TWITTER_HANDLE").expect("TWITTER_HANDLE environment variable must be set");

        // Read the search template
        let search_template = fs::read_to_string(SEARCH_TEMPLATE_FILE_PATH)?;

        // Generate search JSON from posts
        let resources_json = Self::generate_search_json(site, &host);

        // Replace placeholders
        let search_html = search_template
            .replace(HOST_PLACEHOLDER, &host)
            .replace(WEBSITE_NAME_PLACEHOLDER, &website_name)
            .replace(WEBSITE_DESCRIPTION_PLACEHOLDER, &website_description)
            .replace(WEBSITE_LOGO_URL_PLACEHOLDER, &website_logo_url)
            .replace(AUTHOR_NAME_PLACEHOLDER, &author_name)
            .replace(TWITTER_HANDLE_PLACEHOLDER, &twitter_handle)
            .replace(RESOURCES_PLACEHOLDER, &resources_json);

        // Create the search directory and write the file
        let current_path = env::current_dir()?.into_os_string();
        let current_path_str = current_path.to_str().ok_or("Invalid path")?;
        let search_dir = format!("{}/build/search", current_path_str);
        fs::create_dir_all(&search_dir)?;

        let search_path = format!("{}/index.html", search_dir);
        let mut file = File::create(search_path)?;
        write!(file, "{}", search_html)?;

        Ok(())
    }
}
