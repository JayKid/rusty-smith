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
        let mut posts = parser::get_posts();

        // Sort posts by date in reverse chronological order (newest first)
        // Date format is "YYYY-MM-DD" so lexicographic comparison works correctly
        posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));

        site.posts = posts;
        Ok(())
    }
} 