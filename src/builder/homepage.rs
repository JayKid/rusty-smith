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
    let website_name: String = dotenv::var("WEBSITE_NAME").expect("WEBSITE_NAME environment variable must be set");
    let website_logo_url: String = dotenv::var("WEBSITE_LOGO_URL").expect("WEBSITE_LOGO_URL environment variable must be set");
    let website_description: String = dotenv::var("WEBSITE_DESCRIPTION").expect("WEBSITE_DESCRIPTION environment variable must be set");
    let author_name: String = dotenv::var("AUTHOR_NAME").expect("AUTHOR_NAME environment variable must be set");
    let mut template_contents = fs::read_to_string(HOMEPAGE_TEMPLATE_FILE_PATH).unwrap();
    template_contents = template_contents
        .replace(&HOST_PLACEHOLDER, &host)
        .replace(&WEBSITE_NAME, &website_name)
        .replace(&WEBSITE_LOGO_URL, &website_logo_url)
        .replace(&WEBSITE_DESCRIPTION, &website_description)
    .replace(&AUTHOR_NAME, &author_name);
    return template_contents;
}

fn get_homepage_markup(posts: &Vec<parser::Post>) -> String {
    let template = get_home_template();
    let posts_markup = get_posts_markup(posts);
    return template.replace(&POST_ITEMS_PLACEHOLDER, &posts_markup);
}

#[cfg(test)]                                                                                                                                                 
mod tests {                                                                                                                                                  
    use super::*;                                                                                                                                            
    use crate::parser::{FrontmatterData, Post};                                                                                                              
    use std::env;                                                                                                                                            
                                                                                                                                                             
    fn setup_test_env() {                                                                                                                                    
        env::set_var("HOST", "https://example.com");                                                                                                         
        env::set_var("WEBSITE_NAME", "Test Blog");                                                                                                           
        env::set_var("WEBSITE_LOGO_URL", "https://example.com/logo.png");                                                                                    
        env::set_var("WEBSITE_DESCRIPTION", "A test blog description");                                                                                      
        env::set_var("AUTHOR_NAME", "Test Author");                                                                                                          
    }                                                                                                                                                        
                                                                                                                                                             
    fn create_test_post(title: &str, date: &str, description: Option<&str>) -> Post {                                                                        
        let frontmatter = FrontmatterData {                                                                                                                  
            title: title.to_string(),                                                                                                                        
            date: date.to_string(),                                                                                                                          
            description: description.map(|s| s.to_string()),
            keywords: Some(String::from(""))                                                                                           
        };                                                                                                                                                   
                                                                                                                                                             
        Post {            
            file_name: "".to_string(),                                                                                                                                   
            frontmatter,                                                                                                                                     
            full_path: "/home/foo/bar".to_string(),                                                                                                             
            permalink: title.to_lowercase().replace(" ", "-"),                                                                                               
            html: "<p>Test content</p>".to_string(),                                                                                                         
        }                                                                                                                                                    
    }                                                                                                                                                        
                                                                                                                                                             
    #[test]                                                                                                                                                  
    fn test_get_posts_markup() {                                                                                                                             
        setup_test_env();                                                                                                                                    
                                                                                                                                                             
        let posts = vec![                                                                                                                                    
            create_test_post(                                                                                                                                
                "Test Post 1",                                                                                                                               
                "2023-01-01",                                                                                                                                
                Some("First post description"),                                                                                                              
            ),                                                                                                                                               
            create_test_post(                                                                                                                                
                "Test Post 2",                                                                                                                               
                "2023-01-02",                                                                                                                                
                Some("Second post description"),                                                                                                             
            ),                                                                                                                                               
        ];                                                                                                                                                   
                                                                                                                                                             
        let markup = get_posts_markup(&posts);                                                                                                               
                                                                                                                                                             
        // Assert the markup contains expected elements                                                                                                      
        assert!(markup.contains("https://example.com/test-post-1/"));                                                                                        
        assert!(markup.contains("https://example.com/test-post-2/"));                                                                                        
        assert!(markup.contains("First post description"));                                                                                                  
        assert!(markup.contains("Second post description"));                                                                                                 
        assert!(markup.contains("Test Post 1"));                                                                                                             
        assert!(markup.contains("Test Post 2"));                                                                                                             
    }                                                                                                                                                        
                                                                                                                                                             
    #[test]                                                                                                                                                  
    fn test_get_home_template() {                                                                                                                            
        setup_test_env();                                                                                                                                    
                                                                                                                                                             
        let template = get_home_template();                                                                                                                  
                                                                                                                                                             
        // Assert the template contains replaced environment variables                                                                                       
        assert!(template.contains("https://example.com"));                                                                                                   
        assert!(template.contains("Test Blog"));                                                                                                             
        assert!(template.contains("https://example.com/logo.png"));                                                                                          
        assert!(template.contains("A test blog description"));                                                                                               
        assert!(template.contains("Test Author"));                                                                                                           
    }                                                                                                                                                        
                                                                                                                                                             
    #[test]                                                                                                                                                  
    fn test_get_homepage_markup() {                                                                                                                          
        setup_test_env();                                                                                                                                    
                                                                                                                                                             
        let posts = vec![                                                                                                                                    
            create_test_post(                                                                                                                                
                "Test Post 1",                                                                                                                               
                "2023-01-01",                                                                                                                                
                Some("First post description"),                                                                                                              
            ),                                                                                                                                               
        ];                                                                                                                                                   
                                                                                                                                                             
        let markup = get_homepage_markup(&posts);                                                                                                            
                                                                                                                                                             
        // Assert the final markup contains both template and post content                                                                                   
        assert!(markup.contains("https://example.com")); // from template                                                                                  
        assert!(markup.contains("Test Blog")); // from template                                                                                            
        assert!(markup.contains("Test Post 1")); // from posts                                                                                             
        assert!(markup.contains("First post description")); // from posts                                                                                  
    }                                                                                                                                                        
                                                                                                                                                             
    #[test]                                                                                                                                                  
    fn test_post_without_description() {                                                                                                                     
        setup_test_env();                                                                                                                                    
                                                                                                                                                             
        let posts = vec![                                                                                                                                    
            create_test_post(                                                                                                                                
                "Test Post No Description",                                                                                                                  
                "2023-01-01",                                                                                                                                
                None,                                                                                                                                        
            ),                                                                                                                                               
        ];                                                                                                                                                   
                                                                                                                                                             
        let markup = get_posts_markup(&posts);                                                                                                               
                                                                                                                                                             
        // Assert the markup handles missing description correctly                                                                                           
        assert!(markup.contains("Test Post No Description"));                                                                                                
        assert!(markup.contains("")); // Empty description                                                                                                   
    }                                                                                                                                                        
} 