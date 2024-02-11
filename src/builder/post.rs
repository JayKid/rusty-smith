use std::fs;

use crate::parser;

use super::{create_file, get_build_dir};

fn get_permalink_from_title(post_title: String) -> String {
    let lowercase = post_title.to_lowercase();
    return str::replace(&lowercase, " ", "-");
}

fn get_post_full_path(post: &parser::Post) -> String {
    let build_path = get_build_dir();
    let post_dir_name = get_permalink_from_title(String::from(&post.frontmatter.title));
    let full_dir_path = format!("{}/{}", build_path, post_dir_name);
    return full_dir_path;
}

fn create_post_file(post: &parser::Post) -> () {
    let full_path = get_post_full_path(&post);
    let file_path = format!("{}/index.html", full_path);
    create_file(&post.html, &file_path)
}

pub fn create_post_dir_and_file(post: &parser::Post) -> () {
    let full_dir_path = get_post_full_path(&post);

    match fs::create_dir(full_dir_path) {
        Ok(_) => {
            // println!("created dir succesfully");
        }
        Err(msg) => {
            println!("failed to create directory, see error: {}", msg);
        }
    }

    create_post_file(post);
}
