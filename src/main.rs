use serde::Deserialize;
use std::fs;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct Framework {
    name: String,
    list_cmd: String,
    run_cmd: String,
}

fn command_exists(cmd: &str) -> bool {
    let exe = cmd.split_whitespace().next().unwrap();
    which::which(exe).is_ok()
}

fn filter_available_frameworks(frameworks: Vec<Framework>) -> Vec<Framework> {
    frameworks
        .into_iter()
        .filter(|fw| {
            let list_ok = command_exists(&fw.list_cmd);
            let run_ok = command_exists(&fw.run_cmd);
            list_ok && run_ok
        })
        .collect()
}

fn list_all_tests(framework: &Framework) {
    println!("Framework: {}", framework.name);

    let output = Command::new("sh")
        .arg("-c")
        .arg(&framework.list_cmd)
        .output()
        .expect("Failed to list tests");

    println!("Tests:\n{}", String::from_utf8_lossy(&output.stdout));
}

fn run_all_tests(framework: &Framework) {
    println!("Running all tests for {}", framework.name);

    let status = Command::new("sh")
        .arg("-c")
        .arg(&framework.run_cmd)
        .status()
        .expect("Failed to run tests");

    println!("Exit status: {}", status);
}

fn main() {
    let json = fs::read_to_string("src/frameworks.json").expect("Failed to read frameworks.json");
    let frameworks: Vec<Framework> = serde_json::from_str(&json).expect("Invalid JSON format");

    let available_frameworks: Vec<Framework> = filter_available_frameworks(frameworks);

    for framework in &available_frameworks {
        list_all_tests(framework);
    }

    for framework in &available_frameworks {
        run_all_tests(framework);
    }
}
