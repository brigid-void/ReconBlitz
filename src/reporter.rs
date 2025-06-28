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
                (
                    tool.clone(),
                    ScanReport {
                        tool: tool.clone(),
                        output: result.output.clone(),
                        success: result.success,
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
    html.push_str("</style>\n</head>\n<body>\n");
    html.push_str(&format!("<h1>ReconBlitz Report</h1>\n<p>Generated at: {}</p>\n", timestamp));

    for (tool, result) in results {
        html.push_str(&format!("<div class=\"tool\"><h2>{}</h2>", tool));
        html.push_str(&format!("<p>Status: <span class=\"{}\">{}</span></p>", 
            if result.success { "success" } else { "failure" },
            if result.success { "Success" } else { "Failure" }
        ));
        html.push_str(&format!("<pre>{}</pre>", html_escape::encode_text(&result.output)));
        html.push_str("</div>\n");
    }

    html.push_str("</body>\n</html>");
    html
}
