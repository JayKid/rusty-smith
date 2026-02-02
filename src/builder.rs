use crate::plugin::{PluginPipeline, Site};
use crate::plugins::*;

pub fn build() -> Result<(), Box<dyn std::error::Error>> {
    let mut site = Site::new();
    let mut pipeline = PluginPipeline::new();

    // Add plugins in the order they should run
    pipeline.add_plugin(BuildPlugin::new());
    pipeline.add_plugin(PostsPlugin::new());
    pipeline.add_plugin(PostPlugin::new());
    pipeline.add_plugin(HomepagePlugin::new());
    pipeline.add_plugin(PagesPlugin::new());
    pipeline.add_plugin(FeedPlugin::new());
    pipeline.add_plugin(SitemapPlugin::new());

    // Run the pipeline
    pipeline.run(&mut site)?;

    Ok(())
}
