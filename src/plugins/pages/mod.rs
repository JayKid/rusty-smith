use std::env;
use std::fs::{self, DirEntry, File};
use std::io::Write;

use dotenv;
use markdown::{mdast::Node, CompileOptions, Constructs, Options, ParseOptions};

use crate::plugin::{Page, Plugin, Site};

const PAGES_DIR: &str = "pages";
const PAGE_TEMPLATE_FILE_PATH: &str = "./assets/templates/page.html";

// Template placeholders
const PAGE_TITLE_PLACEHOLDER: &str = "{page_title}";
const PAGE_DESCRIPTION_PLACEHOLDER: &str = "{page_description}";
const PAGE_CONTENT_PLACEHOLDER: &str = "{page_content}";
const PAGE_URL_PLACEHOLDER: &str = "{page_url}";
const PAGE_SLUG_PLACEHOLDER: &str = "{page_slug}";
const HOST_PLACEHOLDER: &str = "{host}";
const WEBSITE_NAME_PLACEHOLDER: &str = "{website_name}";
const WEBSITE_LOGO_URL_PLACEHOLDER: &str = "{website_logo_url}";
const AUTHOR_NAME_PLACEHOLDER: &str = "{author_name}";

pub struct PagesPlugin;

impl PagesPlugin {
    pub fn new() -> Self {
        PagesPlugin
    }

    fn parse_html(markdown_content: &str) -> String {
        let parse_options = Options {
            compile: CompileOptions {
                allow_dangerous_html: true,
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
        markdown::to_html_with_options(markdown_content, &parse_options).unwrap_or_default()
    }

    fn parse_frontmatter(markdown_content: &str) -> (String, Option<String>) {
        let parse_options = ParseOptions {
            constructs: Constructs {
                frontmatter: true,
                ..Constructs::gfm()
            },
            ..ParseOptions::default()
        };

        let ast = markdown::to_mdast(markdown_content, &parse_options).ok();

        if let Some(node) = ast {
            if let Some(children) = node.children() {
                if let Some(Node::Yaml(yaml)) = children.first() {
                    let parsed: std::collections::BTreeMap<String, String> =
                        serde_yaml::from_str(&yaml.value).unwrap_or_default();

                    let title = parsed.get("title").cloned().unwrap_or_default();
                    let description = parsed.get("description").cloned();

                    return (title, description);
                }
            }
        }

        (String::new(), None)
    }

    fn get_slug_from_filename(filename: &str) -> String {
        filename.trim_end_matches(".md").to_string()
    }

    fn parse_page(entry: DirEntry) -> Option<Page> {
        let path = entry.path();
        let filename = entry.file_name().to_str()?.to_string();

        if !filename.ends_with(".md") {
            return None;
        }

        let content = fs::read_to_string(&path).ok()?;
        let (title, description) = Self::parse_frontmatter(&content);
        let html = Self::parse_html(&content);
        let slug = Self::get_slug_from_filename(&filename);

        Some(Page {
            title,
            description,
            slug,
            html,
        })
    }
}

impl Plugin for PagesPlugin {
    fn name(&self) -> &str {
        "pages"
    }

    fn run(&self, site: &mut Site) -> Result<(), Box<dyn std::error::Error>> {
        // Get environment variables
        let host = dotenv::var("HOST").expect("HOST environment variable must be set");
        let website_name =
            dotenv::var("WEBSITE_NAME").expect("WEBSITE_NAME environment variable must be set");
        let website_logo_url = dotenv::var("WEBSITE_LOGO_URL")
            .expect("WEBSITE_LOGO_URL environment variable must be set");
        let author_name =
            dotenv::var("AUTHOR_NAME").expect("AUTHOR_NAME environment variable must be set");

        // Read the page template
        let page_template = fs::read_to_string(PAGE_TEMPLATE_FILE_PATH)?;

        // Get build directory
        let current_path = env::current_dir()?.into_os_string();
        let current_path_str = current_path.to_str().ok_or("Invalid path")?;
        let build_dir = format!("{}/build", current_path_str);

        // Read and process all pages
        let pages_dir = fs::read_dir(PAGES_DIR)?;

        for entry in pages_dir.flatten() {
            if let Some(page) = Self::parse_page(entry) {
                // Create output directory
                let page_dir = format!("{}/{}", build_dir, page.slug);
                fs::create_dir_all(&page_dir)?;

                // Apply template
                let page_url = format!("{}/{}/", host, page.slug);
                let page_html = page_template
                    .replace(PAGE_TITLE_PLACEHOLDER, &page.title)
                    .replace(
                        PAGE_DESCRIPTION_PLACEHOLDER,
                        page.description.as_deref().unwrap_or(""),
                    )
                    .replace(PAGE_CONTENT_PLACEHOLDER, &page.html)
                    .replace(PAGE_URL_PLACEHOLDER, &page_url)
                    .replace(PAGE_SLUG_PLACEHOLDER, &page.slug)
                    .replace(HOST_PLACEHOLDER, &host)
                    .replace(WEBSITE_NAME_PLACEHOLDER, &website_name)
                    .replace(WEBSITE_LOGO_URL_PLACEHOLDER, &website_logo_url)
                    .replace(AUTHOR_NAME_PLACEHOLDER, &author_name);

                // Write output file
                let output_path = format!("{}/index.html", page_dir);
                let mut file = File::create(output_path)?;
                write!(file, "{}", page_html)?;

                // Store page in site for sitemap
                site.pages.push(page);
            }
        }

        Ok(())
    }
}
