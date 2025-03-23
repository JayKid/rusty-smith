use crate::plugin::{Plugin, Site};
use std::{env, fs, path::Path};

pub struct BuildPlugin;

impl BuildPlugin {
    pub fn new() -> Self {
        BuildPlugin
    }

    fn get_build_dir() -> String {
        let binding = env::current_dir().unwrap().into_os_string();
        let current_path = binding.to_str().unwrap();
        format!("{}/build/", current_path)
    }

    fn clean_build_dir(&self) -> Result<(), Box<dyn std::error::Error>> {
        let build_path = Self::get_build_dir();
        if Path::new(&build_path).exists() {
            fs::remove_dir_all(build_path)?;
        }
        Ok(())
    }

    fn create_build_dir(&self) -> Result<(), Box<dyn std::error::Error>> {
        let build_path = Self::get_build_dir();
        fs::create_dir_all(build_path)?;
        Ok(())
    }

    fn copy_assets(&self) -> Result<(), Box<dyn std::error::Error>> {
        let build_dir = Self::get_build_dir();
        let styles_source_path = format!("{}/{}", build_dir, "../public/css/styles.css");
        let styles_final_dir_path = format!("{}/{}", build_dir, "css");
        let styles_final_file_path = format!("{}/styles.css", &styles_final_dir_path);

        fs::create_dir_all(&styles_final_dir_path)?;
        fs::copy(styles_source_path, styles_final_file_path)?;
        Ok(())
    }
}

impl Plugin for BuildPlugin {
    fn name(&self) -> &str {
        "build"
    }

    fn run(&self, _site: &mut Site) -> Result<(), Box<dyn std::error::Error>> {
        self.clean_build_dir()?;
        self.create_build_dir()?;
        self.copy_assets()?;
        Ok(())
    }
} 