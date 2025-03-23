use std::collections::HashMap;
use crate::parser::Post;

/// Represents the site's metadata and content during the build process
#[derive(Debug)]
pub struct Site {
    pub posts: Vec<Post>,
    pub metadata: HashMap<String, String>,
}

impl Site {
    pub fn new() -> Self {
        Site {
            posts: Vec::new(),
            metadata: HashMap::new(),
        }
    }
}

/// The core plugin trait that all plugins must implement
pub trait Plugin {
    fn name(&self) -> &str;
    fn run(&self, site: &mut Site) -> Result<(), Box<dyn std::error::Error>>;
}

/// A collection of plugins that will be run in sequence
pub struct PluginPipeline {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginPipeline {
    pub fn new() -> Self {
        PluginPipeline {
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin<P: Plugin + 'static>(&mut self, plugin: P) {
        self.plugins.push(Box::new(plugin));
    }

    pub fn run(&self, site: &mut Site) -> Result<(), Box<dyn std::error::Error>> {
        for plugin in &self.plugins {
            println!("Running plugin: {}", plugin.name());
            plugin.run(site)?;
        }
        Ok(())
    }
} 