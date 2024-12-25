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
    let feed_markup = feed_template.replace(ENTRIES_PLACEHOLDER, &get_feed_entries_contents(posts));

    let path = format!("{}/feed.xml", get_build_dir());

    create_file(&feed_markup, &path);
}

fn get_feed_template() -> String {
    let host: String = dotenv::var("HOST").expect("HOST environment variable must be set");
    let website_name: String =
        dotenv::var("WEBSITE_NAME").expect("WEBSITE_NAME environment variable must be set");
    let author_name: String =
        dotenv::var("AUTHOR_NAME").expect("AUTHOR_NAME environment variable must be set");

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
    return format!("{}T00:00:00.000Z", date);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{FrontmatterData, Post};
    use std::env;

    fn setup_test_env() {
        env::set_var("HOST", "https://example.com");
        env::set_var("WEBSITE_NAME", "Test Blog");
        env::set_var("AUTHOR_NAME", "Test Author");
    }

    fn create_test_post() -> Post {
        Post {
            file_name: "test-post.md".to_string(),
            frontmatter: FrontmatterData {
                date: "2024-01-01".to_string(),
                description: Some("Test description".to_string()),
                keywords: Some("test,keywords".to_string()),
                title: "Test Post".to_string(),
            },
            full_path: "/path/to/test-post.md".to_string(),
            html: "<p>Test content</p>".to_string(),
            permalink: "test-post".to_string(),
        }
    }

    #[test]
    fn test_get_last_updated_time() {
        let date = "2024-01-01".to_string();
        let result = get_last_updated_time(&date);
        assert_eq!(result, "2024-01-01T00:00:00.000Z");
    }

    #[test]
    fn test_get_feed_entries_contents() {
        setup_test_env();

        let test_post = create_test_post();
        let posts = vec![test_post];

        let result = get_feed_entries_contents(&posts);

        // Assert the entry contains the expected elements
        assert!(result.contains("<title>Test Post</title>"));
        assert!(result.contains("<link href=\"https://example.com/test-post/\"/>"));
        assert!(result.contains("<id>https://example.com/test-post/</id>"));
        assert!(result.contains("<updated>2024-01-01T00:00:00.000Z</updated>"));
        assert!(result.contains("<summary>Test description</summary>"));
    }

    #[test]
    fn test_get_feed_template() {
        setup_test_env();

        let result = get_feed_template();

        // Assert the template contains replaced placeholders
        assert!(result.contains("Test Blog"));
        assert!(result.contains("Test Author"));
        assert!(result.contains("https://example.com"));

        // Assert it contains the entries placeholder
        assert!(result.contains("{entries}"));
    }

    #[test]
    fn test_create_feed() {
        setup_test_env();

        let test_post = create_test_post();
        let posts = vec![test_post];

        // Note: This test will actually create a file
        // You might want to use a temporary directory for testing
        create_feed(&posts);

        // Verify the feed.xml file exists in the build directory
        let build_dir = crate::builder::get_build_dir();
        let feed_filepath = format!("{}/feed.xml", build_dir);

        assert!(std::path::Path::new(&feed_filepath).exists());

        // Clean up after test
        let _ = std::fs::remove_file(feed_filepath);
    }
}
