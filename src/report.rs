use crate::analyzer::{Insight, Severity};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use colored::*;

#[derive(Default)]
pub struct Report {
    pub insights: Vec<Insight>,
    pub counts: HashMap<Severity, usize>,
    pub packages_scanned: HashSet<String>,
    pub packages_with_issues: HashSet<String>,
}

impl Report {
    pub fn new() -> Self {
        Self {
            insights: Vec::new(),
            counts: HashMap::new(),
            packages_scanned: HashSet::new(),
            packages_with_issues: HashSet::new(),
        }
    }

    pub fn add(&mut self, insight: Insight) {
        self.packages_with_issues.insert(insight.package.clone());
        self.packages_scanned.insert(insight.package.clone());
    
        self.counts
            .entry(insight.severity.clone())
            .and_modify(|c| *c += 1)
            .or_insert(1);
    
        self.insights.push(insight);
    }

    pub fn is_failure(&self, level: &crate::cli::FailLevel) -> bool {
        match level {
            crate::cli::FailLevel::None => false,
            crate::cli::FailLevel::Warn => self.counts.get(&Severity::Warning).unwrap_or(&0) > &0
                || self.counts.get(&Severity::Critical).unwrap_or(&0) > &0,
            crate::cli::FailLevel::Crit => self.counts.get(&Severity::Critical).unwrap_or(&0) > &0,
        }
    }

    pub fn print_summary(&self) {
        let critical = self.counts.get(&Severity::Critical).unwrap_or(&0);
        let warning = self.counts.get(&Severity::Warning).unwrap_or(&0);
        let info = self.counts.get(&Severity::Info).unwrap_or(&0);
    
        let clean = self.clean_count();
        let total = self.total();
    
        println!("\n===========================");
        println!("{}", "📊 Final Summary:".bold());
        println!("---------------------------");
        println!("🔴 Critical: {}", Self::format_count(critical, "red"));
        println!("🟠 Warnings: {}", Self::format_count(warning, "yellow"));
        println!("🔵 Info:     {}", Self::format_count(info, "blue"));
        println!("✅ Clean:    {}", Self::format_count(&clean, "green"));
        println!("📦 Total:    {}", Self::format_count(&total, "cyan"));
        println!("===========================\n");
    }

    fn format_count(count: &usize, color: &str) -> ColoredString {
        match color {
            "red" => format!("{}", count).red().bold(),
            "yellow" => format!("{}", count).yellow().bold(),
            "blue" => format!("{}", count).blue().bold(),
            "green" => format!("{}", count).green().bold(),
            "cyan" => format!("{}", count).cyan().bold(),
            _ => format!("{}", count).normal(),
        }
    }
    

    pub fn total(&self) -> usize {
        self.packages_scanned.len()
    }
    
    pub fn clean_count(&self) -> usize {
        self.packages_scanned
            .difference(&self.packages_with_issues)
            .count()
    }

    pub fn print_pretty(&self) {
        for insight in &self.insights {
            println!(
                "{} {} {}",
                "-".bold(),
                format_severity(&insight.severity),
                format!("{}: {}", insight.package.cyan(), insight.message)
            );
        }
    }

    pub fn print_json(&self) {
        #[derive(Serialize)]
        struct JsonInsight<'a> {
            package: &'a str,
            message: &'a str,
            severity: &'a Severity,
        }

        let json_output: Vec<_> = self
            .insights
            .iter()
            .map(|i| JsonInsight {
                package: &i.package,
                message: &i.message,
                severity: &i.severity,
            })
            .collect();

        println!("{}", serde_json::to_string_pretty(&json_output).unwrap());
    }
}

fn format_severity(sev: &Severity) -> ColoredString {
    match sev {
        Severity::Info => "INFO".blue(),
        Severity::Warning => "WARN".yellow(),
        Severity::Critical => "CRIT".red().bold(),
    }
}

