use serde::Deserialize;
use std::fs;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct Framework {
    name: String,
    list_cmd: String,
    run_cmd: String,
}

fn main() {
    let json = fs::read_to_string("src/frameworks.json").expect("Failed to read frameworks.json");
    let frameworks: Vec<Framework> = serde_json::from_str(&json).expect("Invalid JSON format");

    for framework in &frameworks {
        println!("Framework: {}", framework.name);

        let output = Command::new("sh")
            .arg("-c")
            .arg(&framework.list_cmd)
            .output()
            .expect("Failed to list tests");

        println!("Tests:\n{}", String::from_utf8_lossy(&output.stdout));
    }

    for framework in &frameworks {
        println!("Running all tests for {}", framework.name);

        let status = Command::new("sh")
            .arg("-c")
            .arg(&framework.run_cmd)
            .status()
            .expect("Failed to run tests");

        println!("Exit status: {}", status);
    }
}
