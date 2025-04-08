mod analyzer;
mod cli;
mod metadata;
mod parser;
mod report;
mod utils;

use crate::report::Report;
use std::collections::HashMap;
use indicatif::{ProgressBar, ProgressStyle};
use crate::analyzer::analyze_package;

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
    deps: std::collections::HashMap<String, String>,
    label: &str,
    report: &mut Report,
) {
    let total = deps.len();
    let bar = ProgressBar::new(total as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "🔍 Analyzing [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} packages",
        )
        .unwrap()
        .progress_chars("=>-"),
    );

    bar.println(format!("📦 Starting analysis for {}...", label));

    for (pkg, _) in deps {
        match metadata::fetch_metadata(&pkg).await {
            Ok(meta) => {
                let insights = analyze_package(&meta);

                if insights.is_empty() {
                    report.packages_scanned.insert(pkg.clone());
                } else {
                    for insight in insights {
                        report.add(insight);
                    }
                }
            }
            Err(err) => {
                bar.println(format!("❌ Failed to fetch {}: {}", pkg, err));
            }
        }

        bar.inc(1);
    }

    bar.finish_with_message(format!("✅ Finished analyzing {}", label));
}