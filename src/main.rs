mod builder;
mod commands;
mod parser;
mod plugin;
mod plugins;

use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "new" {
        println!("Creating new post...\n");
        let file_path = commands::create_post()?;
        println!("Successfully created new post:");
        println!("{}", file_path);
    } else {
        builder::build()?;
    }

    Ok(())
}
