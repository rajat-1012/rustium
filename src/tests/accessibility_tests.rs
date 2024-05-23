use std::process::Command;
use serde::Serialize;

#[derive(Serialize)]
struct TestResult {
    test_name: String,
    passed: bool,
    error: Option<String>,
}

#[tokio::main]
async fn main() {
    let test_results = vec![
        run_accessibility_test().await,
    ];
    
    generate_report(test_results);
}

async fn run_accessibility_test() -> TestResult {
    let output = Command::new("axe-cli")
        .arg("http://example.com")
        .output()
        .expect("failed to execute axe-cli");

    let passed = output.status.success();

    TestResult {
        test_name: "accessibility_test".to_string(),
        passed,
        error: if passed { None } else { Some(String::from_utf8_lossy(&output.stderr).to_string()) },
    }
}

fn generate_report(results: Vec<TestResult>) {
    let tera = Tera::new("templates/*.html").unwrap();
    let mut context = Context::new();
    context.insert("results", &results);

    let rendered = tera.render("report.html", &context).unwrap();
    std::fs::write("reports/accessibility_report.html", rendered).unwrap();
}
