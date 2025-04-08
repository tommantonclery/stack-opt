mod analyzer;
mod cli;
mod metadata;
mod parser;
mod report;
mod utils;

use crate::report::Report;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let args = cli::parse_args();
    let mut report = Report::new();

    println!("🔍 Analyzing project at: {}", args.path);

    match parser::load_package_json(&args.path) {
        Ok(package_json) => {
            if let Some(deps) = package_json.dependencies {
                println!("\n📦 Analyzing dependencies...");
                analyze_set(deps, "dependencies", &mut report).await;
            }

            if !args.no_dev {
                if let Some(dev_deps) = package_json.dev_dependencies {
                    println!("\n🔧 Analyzing devDependencies...");
                    analyze_set(dev_deps, "devDependencies", &mut report).await;
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }

    // Output results
    if args.json {
        report.print_json();
    } else {
        report.print_pretty();
        report.print_summary();
    }

    // Handle fail-on flag
    if report.is_failure(&args.fail_on) {
        std::process::exit(1);
    }
}

async fn analyze_set(
    deps: HashMap<String, String>,
    label: &str,
    report: &mut Report,
) {
    for (pkg, _) in deps {
        println!("\n📦 Fetching metadata for {} [{}]...", pkg, label);

        match metadata::fetch_metadata(&pkg).await {
            Ok(meta) => {
                let insights = analyzer::analyze_package(&meta);

                if insights.is_empty() {
                    // Track clean package
                    report.packages_scanned.insert(pkg.clone());
                    println!("✅ No issues found for {}", pkg);
                } else {
                    for insight in insights {
                        report.add(insight);
                    }
                }
            }
            Err(err) => {
                eprintln!("❌ Error for {}: {}", pkg, err);
            }
        }
    }
}
