use chrono::Local;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

const POSTS_DIR: &str = "posts";

const POST_TEMPLATE: &str = r#"---
title: "New post title"
description: The description
keywords: keyword
# permalink: if-needed
date: {date}
# publish: draft
---

<section>

## First subtitle

Paragraph contents

</section>
"#;

pub fn create_post() -> Result<String, Box<dyn std::error::Error>> {
    // Ensure posts directory exists
    let posts_path = Path::new(POSTS_DIR);
    if !posts_path.exists() {
        fs::create_dir_all(posts_path)?;
    }

    // Get current date in YYYY-MM-DD format
    let date = Local::now().format("%Y-%m-%d").to_string();

    // Create filename and content
    let filename = format!("{}-post-title.md", date);
    let content = POST_TEMPLATE.replace("{date}", &date);

    // Write the file
    let file_path = posts_path.join(&filename);
    let mut file = File::create(&file_path)?;
    write!(file, "{}", content)?;

    Ok(file_path.to_string_lossy().to_string())
}
