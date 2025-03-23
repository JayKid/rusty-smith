use crate::plugin::{Plugin, Site};
use crate::parser;

pub struct PostsPlugin;

impl PostsPlugin {
    pub fn new() -> Self {
        PostsPlugin
    }
}

impl Plugin for PostsPlugin {
    fn name(&self) -> &str {
        "posts"
    }

    fn run(&self, site: &mut Site) -> Result<(), Box<dyn std::error::Error>> {
        // Read posts from the posts directory
        let posts = parser::get_posts();
        site.posts = posts;
        Ok(())
    }
} 