use serde::Serialize;
use std::collections::HashMap;
use chrono::Utc;
use html_escape;

#[derive(Serialize)]
pub struct UnifiedReport {
    target: String,
    scans: HashMap<String, ScanReport>,
}

#[derive(Serialize)]
pub struct ScanReport {
    tool: String,
    output: String,
    success: bool,
}

pub fn generate_json_report(results: &HashMap<String, crate::scanner::ScanResult>) -> String {
    let unified = UnifiedReport {
        target: "".to_string(), // Will be set by caller
        scans: results
            .iter()
            .map(|(tool, result)| {
                let (output, success) = match result {
                    crate::scanner::ScanResult::Success(s) => (s.clone(), true),
                    crate::scanner::ScanResult::Failure(s) => (s.clone(), false),
                    crate::scanner::ScanResult::Error(e) => (e.clone(), false),
                    crate::scanner::ScanResult::Timeout => ("Command timed out".to_string(), false),
                };
                (
                    tool.clone(),
                    ScanReport {
                        tool: tool.clone(),
                        output,
                        success,
                    },
                )
            })
            .collect(),
    };
    serde_json::to_string_pretty(&unified).unwrap()
}

pub fn generate_html_report(results: &HashMap<String, crate::scanner::ScanResult>) -> String {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let mut html = String::new();
    html.push_str(&format!("<!DOCTYPE html>\n<html>\n<head>\n"));
    html.push_str("<title>ReconBlitz Report</title>\n");
    html.push_str("<style>\n");
    html.push_str("body { font-family: sans-serif; }\n");
    html.push_str(".tool { background-color: #f0f0f0; padding: 10px; margin-bottom: 10px; }\n");
    html.push_str(".success { color: green; }\n");
    html.push_str(".failure { color: red; }\n");
    html.push_str(".timeout { color: orange; }\n");
    html.push_str("</style>\n</head>\n<body>\n");
    html.push_str(&format!("<h1>ReconBlitz Report</h1>\n<p>Generated at: {}</p>\n", timestamp));

    for (tool, result) in results {
        let (status_class, status_text, output) = match result {
            crate::scanner::ScanResult::Success(s) => ("success", "Success", s.clone()),
            crate::scanner::ScanResult::Failure(s) => ("failure", "Failure", s.clone()),
            crate::scanner::ScanResult::Error(e) => ("failure", "Error", e.clone()),
            crate::scanner::ScanResult::Timeout => ("timeout", "Timeout", "Command timed out".to_string()),
        };

        html.push_str(&format!("<div class=\"tool\"><h2>{}</h2>", tool));
        html.push_str(&format!("<p>Status: <span class=\"{}\">{}</span></p>", status_class, status_text));
        html.push_str(&format!("<pre>{}</pre>", html_escape::encode_text(&output)));
        html.push_str("</div>\n");
    }

    html.push_str("</body>\n</html>");
    html
}
