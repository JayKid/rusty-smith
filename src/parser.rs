use std::{
    collections::BTreeMap,
    fs::{self, DirEntry},
};

use markdown::{mdast::Node, CompileOptions, Constructs, Options, ParseOptions};

#[derive(Debug)]
pub struct FrontmatterData {
    pub date: String,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub light_theme: bool,
    pub publish: Option<String>,
    pub title: String,
}

impl FrontmatterData {
    /// Returns true if the post is a draft (publish == "draft")
    pub fn is_draft(&self) -> bool {
        self.publish.as_deref() == Some("draft")
    }

    /// Returns the CSS class for the theme ("light-theme" or "")
    pub fn theme_class(&self) -> &str {
        if self.light_theme {
            "light-theme"
        } else {
            ""
        }
    }
}

#[derive(Debug)]
pub struct Post {
    pub file_name: String,
    pub frontmatter: FrontmatterData,
    pub full_path: String,
    pub html: String,
    pub permalink: String,
}

const POSTS_FILE_PATH: &str = "posts";

fn get_permalink_from_title(post_title: &String) -> String {
    let lowercase = post_title.to_lowercase();
    return str::replace(&lowercase, " ", "-");
}

fn parse_html(post_markdown: &String) -> String {
    let parse_options = Options {
        compile: CompileOptions {
            allow_dangerous_html: true, // I need it for my mixed Markdown + HTML post style
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
    let html = markdown::to_html_with_options(post_markdown, &parse_options).unwrap();
    return html;
}

fn parse_frontmatter_data(frontmatter_data: Node) -> Result<FrontmatterData, String> {
    match frontmatter_data {
        Node::Yaml(value) => {
            let parsed_ast: BTreeMap<String, String> = serde_yaml::from_str(&value.value)
                .map_err(|e| format!("YAML parsing error: {}", e))?;

            let title_src = parsed_ast.get("title");
            let parsed_title =
                title_src.ok_or_else(|| "Missing required field: title".to_string())?;

            let date_src = parsed_ast.get("date");
            let parsed_date = date_src.ok_or_else(|| "Missing required field: date".to_string())?;

            let parsed_keywords = parsed_ast.get("keywords");
            let parsed_description = parsed_ast.get("description");
            let parsed_publish = parsed_ast.get("publish");
            let parsed_light_theme = parsed_ast
                .get("lightTheme")
                .map(|v| v == "true")
                .unwrap_or(false);

            Ok(FrontmatterData {
                title: parsed_title.to_string(),
                description: parsed_description.cloned(),
                keywords: parsed_keywords.cloned(),
                light_theme: parsed_light_theme,
                publish: parsed_publish.cloned(),
                date: parsed_date.to_string(),
            })
        }
        _ => Err("there was an error parsing frontmatter data".to_owned()),
    }
}

fn parse_post(post_path: DirEntry) -> Result<Post, String> {
    let custom = Constructs {
        frontmatter: true,
        ..Constructs::gfm()
    };
    let parse_options = ParseOptions {
        constructs: custom,
        ..ParseOptions::default()
    };

    let full_path = &post_path
        .path()
        .to_str()
        .unwrap_or("file_path error")
        .to_owned();
    let file_name = post_path
        .file_name()
        .to_str()
        .unwrap_or("file_name error")
        .to_owned();
    let post_markdown =
        fs::read_to_string(String::from(full_path)).expect("should have read the file");

    let parsed_post_html = parse_html(&post_markdown);

    let parsed_ast_from_post = markdown::to_mdast(&post_markdown, &parse_options)
        .map_err(|err| format!("Failed to parse markdown: {}", err))?;
    let frontmatter_data = parsed_ast_from_post
        .children()
        .ok_or_else(|| "No children found in markdown".to_string())?
        .clone()
        .into_iter()
        .nth(0)
        .ok_or_else(|| "No frontmatter found".to_string())?;

    let post_frontmatter = parse_frontmatter_data(frontmatter_data)?;
    let permalink = get_permalink_from_title(&post_frontmatter.title);

    let new_post = Post {
        file_name,
        frontmatter: post_frontmatter,
        full_path: String::from(full_path),
        html: parsed_post_html,
        permalink: permalink,
    };
    return Ok(new_post);
}

pub fn get_posts() -> Vec<Post> {
    let error_reading_files_message = format!("Error reading files at {}", POSTS_FILE_PATH);
    let post_paths = fs::read_dir(POSTS_FILE_PATH).expect(&error_reading_files_message);

    let mut parsed_posts = Vec::<Post>::new();

    for post_path in post_paths {
        match post_path {
            Ok(post_dir_entry) => {
                let parsed_post = parse_post(post_dir_entry);
                match parsed_post {
                    Ok(post) => {
                        parsed_posts.push(post);
                    }
                    Err(error_message) => {
                        println!("There was an error with the post {}", error_message);
                    }
                }
            }
            Err(error_message) => {
                println!("There was an error with the post: {}", error_message);
            }
        }
    }

    return parsed_posts;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    // Helper function to create a temporary markdown file
    fn create_test_markdown_file(temp_dir: &TempDir, content: &str) -> DirEntry {
        let file_path = temp_dir.path().join("test-post.md");
        let mut file = File::create(&file_path).unwrap();
        write!(file, "{}", content).unwrap();
        fs::read_dir(temp_dir.path())
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
    }

    #[test]
    fn test_get_permalink_from_title() {
        assert_eq!(
            get_permalink_from_title(&String::from("Hello World")),
            "hello-world"
        );
        assert_eq!(
            get_permalink_from_title(&String::from("Test Title 123")),
            "test-title-123"
        );
        assert_eq!(
            get_permalink_from_title(&String::from("UPPER CASE")),
            "upper-case"
        );
    }

    #[test]
    fn test_parse_html() {
        let markdown = String::from("# Test\n\nThis is a **test**.");
        let html = parse_html(&markdown);
        assert!(html.contains("<h1>"));
        assert!(html.contains("Test"));
        assert!(html.contains("<strong>test</strong>"));
    }

    #[test]
    fn test_parse_frontmatter_data() {
        let yaml = Node::Yaml(markdown::mdast::Yaml {
            value: String::from(
                r#"title: Test Post
date: 2024-01-01
description: Test description
keywords: test,keywords"#,
            ),
            position: None,
        });

        let result = parse_frontmatter_data(yaml).unwrap();

        assert_eq!(result.title, "Test Post");
        assert_eq!(result.date, "2024-01-01");
        assert_eq!(result.description, Some("Test description".to_string()));
        assert_eq!(result.keywords, Some("test,keywords".to_string()));
        assert_eq!(result.publish, None);
        assert!(!result.is_draft());
        assert!(!result.light_theme);
        assert_eq!(result.theme_class(), "");
    }

    #[test]
    fn test_parse_frontmatter_data_with_light_theme() {
        let yaml = Node::Yaml(markdown::mdast::Yaml {
            value: String::from(
                r#"title: Light Theme Post
date: 2024-01-01
lightTheme: true"#,
            ),
            position: None,
        });

        let result = parse_frontmatter_data(yaml).unwrap();

        assert!(result.light_theme);
        assert_eq!(result.theme_class(), "light-theme");
    }

    #[test]
    fn test_parse_frontmatter_data_with_draft() {
        let yaml = Node::Yaml(markdown::mdast::Yaml {
            value: String::from(
                r#"title: Draft Post
date: 2024-01-01
publish: draft"#,
            ),
            position: None,
        });

        let result = parse_frontmatter_data(yaml).unwrap();

        assert_eq!(result.title, "Draft Post");
        assert_eq!(result.publish, Some("draft".to_string()));
        assert!(result.is_draft());
    }

    #[test]
    fn test_parse_frontmatter_data_with_published() {
        let yaml = Node::Yaml(markdown::mdast::Yaml {
            value: String::from(
                r#"title: Published Post
date: 2024-01-01
publish: published"#,
            ),
            position: None,
        });

        let result = parse_frontmatter_data(yaml).unwrap();

        assert_eq!(result.publish, Some("published".to_string()));
        assert!(!result.is_draft());
    }

    #[test]
    fn test_parse_frontmatter_data_missing_required_fields() {
        let yaml = Node::Yaml(markdown::mdast::Yaml {
            value: String::from("description: Test description"),
            position: None,
        });

        let result = parse_frontmatter_data(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_post() {
        let temp_dir = TempDir::new().unwrap();
        let content = r#"---
title: Test Post
date: 2024-01-01
description: Test description
keywords: test,keywords
---

# Test Content

This is test content."#;

        let dir_entry = create_test_markdown_file(&temp_dir, content);
        let result = parse_post(dir_entry).unwrap();

        assert_eq!(result.frontmatter.title, "Test Post");
        assert_eq!(result.frontmatter.date, "2024-01-01");
        assert_eq!(result.permalink, "test-post");
        assert!(result.html.contains("<h1>Test Content</h1>"));
    }

    #[test]
    fn test_parse_post_invalid_frontmatter() {
        let temp_dir = TempDir::new().unwrap();
        // Make sure we have proper YAML delimiters and structure, but with invalid content
        let content = r#"---
title: Test Post
date: 2024-01-01
description: Test description
keywords: [test,keywords]
---

# Test Content"#;

        let dir_entry = create_test_markdown_file(&temp_dir, content);
        let result = parse_post(dir_entry);
        assert!(result.is_err());
        if let Err(err) = result {
            // The error should come from trying to parse the invalid YAML structure
            assert!(err.contains("YAML parsing error"));
        }
    }

    // Note: We're not testing get_posts() directly because it depends on the actual filesystem
    // and the POSTS_FILE_PATH constant. In a real application, you might want to make the
    // posts directory path configurable for testing purposes.
}
