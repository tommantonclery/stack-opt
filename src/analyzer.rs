use crate::metadata::PackageMeta;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub enum Severity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug)]
pub struct Insight {
    pub package: String,
    pub message: String,
    pub severity: Severity,
}

pub fn analyze_package(meta: &PackageMeta) -> Vec<Insight> {
    let mut insights = vec![];

    // 1. Deprecation
    if let Some(reason) = &meta.deprecated {
        insights.push(Insight {
            package: meta.name.clone(),
            message: format!("⚠️ Deprecated: {}", reason),
            severity: Severity::Critical,
        });
    }

    // 2. Large Size
    if let Some(size) = meta.unpacked_size {
        if size > 200_000 {
            insights.push(Insight {
                package: meta.name.clone(),
                message: format!("🐘 Large size (~{:.1} KB)", size as f64 / 1024.0),
                severity: Severity::Warning,
            });
        }
    }

    // 3. Stale
    if let Some(updated) = meta.last_updated {
        let now = chrono::Utc::now().naive_utc().date();
        let age = (now - updated).num_days();
        if age > 365 {
            insights.push(Insight {
                package: meta.name.clone(),
                message: format!("🕒 Last updated {} days ago", age),
                severity: Severity::Warning,
            });
        }
    }

    // 4. Low Maintainer Count
    if meta.maintainer_count <= 1 {
        insights.push(Insight {
            package: meta.name.clone(),
            message: "👤 Only one maintainer".to_string(),
            severity: Severity::Info,
        });
    }

    insights
}
