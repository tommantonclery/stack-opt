use serde::Deserialize;
use std::{collections::HashMap, fs};
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct PackageJson {
    pub dependencies: Option<HashMap<String, String>>,
    pub devDependencies: Option<HashMap<String, String>>,
}

pub fn load_package_json(path: &str) -> Result<PackageJson, String> {
    let file_path = Path::new(path).join("package.json");

    let data = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read package.json: {}", e))?;

    let parsed: PackageJson = serde_json::from_str(&data)
        .map_err(|e| format!("Invalid JSON format in package.json: {}", e))?;

    Ok(parsed)
}
