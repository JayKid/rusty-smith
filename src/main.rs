mod builder;
mod parser;
mod plugin;
mod plugins;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    builder::build()?;
    Ok(())
}
