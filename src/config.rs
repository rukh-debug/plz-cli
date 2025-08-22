use colored::Colorize;
use std::{env, io::Write, process::exit};

pub struct Config {
    pub ollama_url: String,
    pub ollama_model: String,
    pub shell: String,
}

impl Config {
    pub fn new() -> Self {
        let ollama_url = env::var("OLLAMA_URL").unwrap_or_else(|_| {
            println!("{}", "Warning: OLLAMA_URL environment variable not set. Using default: https://ollama.rukh.me".yellow());
            String::from("https://ollama.rukh.me")
        });
        
        let ollama_model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| {
            println!("{}", "Error: This program requires an Ollama model to be specified. Please set the OLLAMA_MODEL environment variable (e.g., qwen2.5-coder:7b, codellama, etc.)".red());
            println!("{}", "Available models can be listed with: ollama list".bright_black());
            exit(1);
        });
        
        let shell = env::var("SHELL").unwrap_or_else(|_| String::new());

        Self { ollama_url, ollama_model, shell }
    }

    pub fn write_to_history(&self, code: &str) {
        let history_file = match self.shell.as_str() {
            "/bin/bash" => std::env::var("HOME").unwrap() + "/.bash_history",
            "/bin/zsh" => std::env::var("HOME").unwrap() + "/.zsh_history",
            _ => return,
        };

        if let Ok(mut file) = std::fs::OpenOptions::new()
            .append(true)
            .open(history_file)
        {
            let _ = file.write_all(format!("{code}\n").as_bytes());
        }
    }
}
