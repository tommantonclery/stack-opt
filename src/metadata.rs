use chrono::NaiveDate;
use serde::Deserialize;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
pub struct PackageMeta {
    pub name: String,
    pub deprecated: Option<String>,
    pub unpacked_size: Option<u64>,
    pub last_updated: Option<NaiveDate>,
    pub maintainer_count: usize,
}

#[derive(Deserialize, Debug)]
struct NpmResponse {
    #[serde(rename = "dist-tags")]
    dist_tags: DistTags,
    versions: HashMap<String, VersionInfo>,
    time: HashMap<String, String>,
    maintainers: Option<Vec<Maintainer>>,
}

#[derive(Deserialize, Debug)]
struct DistTags {
    latest: String,
}

#[derive(Deserialize, Debug)]
struct VersionInfo {
    deprecated: Option<String>,
    dist: Option<DistInfo>,
}

#[derive(Deserialize, Debug)]
struct DistInfo {
    #[serde(rename = "unpackedSize")]
    unpacked_size: Option<u64>,
}

#[derive(Deserialize, Debug)]
struct Maintainer {
    name: String,
}

pub async fn fetch_metadata(package_name: &str) -> Result<PackageMeta, String> {
    let url = format!("https://registry.npmjs.org/{}", package_name);
    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to fetch metadata for {}: {}", package_name, e))?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch {}: HTTP {}", package_name, response.status()));
    }

    let npm_data: NpmResponse = response
        .json()
        .await
        .map_err(|e| format!("Invalid JSON from NPM for {}: {}", package_name, e))?;

    let latest_version = &npm_data.dist_tags.latest;
    let version_info = npm_data.versions.get(latest_version).ok_or_else(|| {
        format!(
            "No metadata found for latest version ({}) of {}",
            latest_version, package_name
        )
    })?;

    let deprecated = version_info.deprecated.clone();
    let unpacked_size = version_info
        .dist
        .as_ref()
        .and_then(|d| d.unpacked_size);

    let last_updated = npm_data.time.get("modified").and_then(|date_str| {
        NaiveDate::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%.fZ").ok()
    });

    let maintainer_count = npm_data.maintainers.as_ref().map(|m| m.len()).unwrap_or(0);

    Ok(PackageMeta {
        name: package_name.to_string(),
        deprecated,
        unpacked_size,
        last_updated,
        maintainer_count,
    })
}
