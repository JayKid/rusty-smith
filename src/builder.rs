mod homepage;
mod post;

use crate::parser;
use std::{
    env,
    fs::{self, File},
    io::Write,
};

fn get_build_dir() -> String {
    let binding = env::current_dir().unwrap().into_os_string();
    let current_path = binding.to_str().unwrap();
    return format!("{}/build/", current_path);
}

fn clean_build_dir() -> () {
    let build_path = get_build_dir();
    let _ = fs::remove_dir_all(build_path);
}

fn create_build_dir() -> () {
    let build_path = get_build_dir();
    match fs::create_dir(build_path) {
        Ok(_) => {}
        Err(msg) => {
            println!("failed to create build dir, {}", msg);
        }
    }
}

fn create_file(contents: &str, file_path: &str) -> () {
    let mut file = File::create(file_path).unwrap();
    match write!(file, "{}", contents) {
        Ok(_) => {
            // println!("created file succesfully");
        }
        Err(msg) => {
            println!("failed to create file, see error: {}", msg);
        }
    }
}

fn add_public_assets_to_build() -> () {
    // TO-DO: Consider using copy_dir crate or similar to avoid this
    let styles_source_path = format!("{}/{}", get_build_dir(), "../public/css/styles.css");
    let styles_final_dir_path = format!("{}/{}", get_build_dir(), "css");
    let styles_final_file_path = format!("{}/styles.css", &styles_final_dir_path);

    fs::create_dir_all(&styles_final_dir_path).expect("failed to create assets dir");

    fs::copy(styles_source_path, styles_final_file_path).expect("failed to create stylesheet");
}

pub fn build(posts: &Vec<parser::Post>) -> () {
    clean_build_dir();
    create_build_dir();

    for post in posts {
        post::create_post_dir_and_file(&post);
    }

    homepage::create_homepage(&posts);

    add_public_assets_to_build();
}
