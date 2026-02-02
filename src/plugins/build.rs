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

    fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if !dst.exists() {
            fs::create_dir_all(dst)?;
        }

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if src_path.is_dir() {
                Self::copy_dir_recursive(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }

        Ok(())
    }

    fn copy_assets(&self) -> Result<(), Box<dyn std::error::Error>> {
        let build_dir = Self::get_build_dir();
        let public_dir = Path::new("public");
        let build_path = Path::new(&build_dir);

        Self::copy_dir_recursive(public_dir, build_path)?;

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