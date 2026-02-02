use std::env;
use std::fs::File;
use std::io::Write;

use dotenv;

use crate::plugin::{Plugin, Site};

pub struct SitemapPlugin;

impl SitemapPlugin {
    pub fn new() -> Self {
        SitemapPlugin
    }
}

impl Plugin for SitemapPlugin {
    fn name(&self) -> &str {
        "sitemap"
    }

    fn run(&self, site: &mut Site) -> Result<(), Box<dyn std::error::Error>> {
        let host = dotenv::var("HOST").expect("HOST environment variable must be set");

        // Build URL entries for all posts
        let mut url_entries = String::new();
        for post in &site.posts {
            url_entries.push_str(&format!(
                "<url><loc>{}/{}/</loc><changefreq>weekly</changefreq><priority>0.5</priority></url>",
                host, post.permalink
            ));
        }

        // Add homepage
        url_entries.push_str(&format!(
            "<url><loc>{}</loc><changefreq>weekly</changefreq><priority>0.5</priority></url>",
            host
        ));

        // Build the complete sitemap XML
        let sitemap_xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">{}</urlset>"#,
            url_entries
        );

        // Write the sitemap file
        let current_path = env::current_dir()?.into_os_string();
        let current_path_str = current_path.to_str().ok_or("Invalid path")?;
        let sitemap_path = format!("{}/build/sitemap.xml", current_path_str);

        let mut file = File::create(sitemap_path)?;
        write!(file, "{}", sitemap_xml)?;

        Ok(())
    }
}
