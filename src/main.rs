#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use bat::PrettyPrinter;
use clap::Parser;
use colored::Colorize;
use config::Config;
use question::{Answer, Question};
use reqwest::blocking::Client;
use serde_json::json;
use spinners::{Spinner, Spinners};
use std::process::Command;

use std::fs::File;
use std::io::{self, BufRead};
// use std::path::Path;

mod config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Description of the command to execute
    prompt: Vec<String>,

    /// Run the generated program without asking for confirmation
    #[clap(short = 'y', long)]
    force: bool,
}

fn main() {
    let cli = Cli::parse();
    let config = Config::new();

    let client = Client::new();
    let mut spinner = Spinner::new(Spinners::BouncingBar, "Generating your command...".into());
    let api_addr = format!("{}/api/chat", config.ollama_url);
    let response = client
        .post(api_addr)
        .json(&json!({
            "model": config.ollama_model,
            "messages": [
                {
                    "role": "system",
                    "content": "Generate a small Bash/Zsh script for the given task. Return ONLY the raw script without any formatting, markdown, or code block indicators. Dont include explanations if not necessary, else include as comments within the script."
                },
                {
                    "role": "user",
                    "content": build_prompt(&cli.prompt.join(" "))
                }
            ],
            "stream": false
        }))
        .header("Content-Type", "application/json")
        .send().unwrap();


    let status_code = response.status();
    if status_code.is_client_error() {
        let response_body = response.json::<serde_json::Value>().unwrap();
        let error_message = response_body.get("error")
            .and_then(|e| e.as_str())
            .unwrap_or("Unknown client error");
        spinner.stop_and_persist(
            "✖".red().to_string().as_str(),
            format!("API error: \"{error_message}\"").red().to_string(),
        );
        std::process::exit(1);
    } else if status_code.is_server_error() {
        spinner.stop_and_persist(
            "✖".red().to_string().as_str(),
            format!("Ollama server is currently experiencing problems. Status code: {status_code}")
                .red()
                .to_string(),
        );
        std::process::exit(1);
    }

    let response_json = response.json::<serde_json::Value>().unwrap();
    let code = response_json["message"]["content"]
        .as_str()
        .unwrap_or_else(|| {
            spinner.stop_and_persist(
                "✖".red().to_string().as_str(),
                "Failed to parse response from Ollama".red().to_string(),
            );
            std::process::exit(1);
        })
        .trim()
        .to_string();

    spinner.stop_and_persist(
        "✔".green().to_string().as_str(),
        "Got some code!".green().to_string(),
    );

    PrettyPrinter::new()
        .input_from_bytes(code.as_bytes())
        .language("bash")
        .grid(true)
        .print()
        .unwrap();

    let should_run = if cli.force {
        true
    } else {
        Question::new(
            ">> Run the generated program? [Y/n]"
                .bright_black()
                .to_string()
                .as_str(),
        )
        .yes_no()
        .until_acceptable()
        .default(Answer::YES)
        .ask()
        .expect("Couldn't ask question.")
            == Answer::YES
    };

    if should_run {
        config.write_to_history(code.as_str());
        spinner = Spinner::new(Spinners::BouncingBar, "Executing...".into());

        let output = Command::new("bash")
            .arg("-c")
            .arg(code.as_str())
            .output()
            .unwrap_or_else(|_| {
                spinner.stop_and_persist(
                    "✖".red().to_string().as_str(),
                    "Failed to execute the generated program.".red().to_string(),
                );
                std::process::exit(1);
            });

        if !output.status.success() {
            spinner.stop_and_persist(
                "✖".red().to_string().as_str(),
                "The program threw an error.".red().to_string(),
            );
            println!("{}", String::from_utf8_lossy(&output.stderr));
            std::process::exit(1);
        }

        spinner.stop_and_persist(
            "✔".green().to_string().as_str(),
            "Command ran successfully".green().to_string(),
        );

        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

fn get_linux_distro() -> Option<String> {
    if let Ok(file) = File::open("/etc/os-release") {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("ID=") {
                    // Extract the distribution ID from the line
                    let distro_id = line[3..].trim_matches('"').to_string();
                    return Some(distro_id);
                }
            }
        }
    }
    None
}

fn build_prompt(prompt: &str) -> String {
    let os_hint = if cfg!(target_os = "macos") {
        " (on macOS)".to_string()
    }
    else if cfg!(target_os = "linux") {
        if let Some(distro) = get_linux_distro() {
            format!(" (on {} Linux)", distro)
        } else {
            " (on Linux)".to_string()
        }
    } else {
        "".to_string()
    };

    format!("{prompt}{os_hint}:\n\n#!/usr/bin/env zsh\n", prompt = prompt, os_hint = os_hint)
}