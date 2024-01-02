use clap::{Arg, Command};
use dirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
mod handler;
#[derive(Debug, Deserialize, Serialize)]
struct Config {
    gpt_api_key: Option<String>,
    gemini_api_key: Option<String>,
}

impl Config {
    fn new() -> Result<Self, io::Error> {
        let config_path = Self::get_config_path();
        if config_path.exists() {
            let mut file = File::open(&config_path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let config: Config = serde_json::from_str(&contents)?;
            Ok(config)
        } else {
            Ok(Config {
                gpt_api_key: None,
                gemini_api_key: None,
            })
        }
    }

    fn save(&self) -> Result<(), io::Error> {
        let config_path = Self::get_config_path();
        println!("config_path :: {}", config_path.to_str().unwrap());
        fs::create_dir_all(config_path.parent().unwrap())?;
        let mut file = File::create(config_path)?;
        let serialized = serde_json::to_string(self)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    fn get_config_path() -> PathBuf {
        let mut base_dir = dirs::home_dir().unwrap();
        println!("base_dir ：{}", base_dir.to_str().unwrap());
        base_dir.push(".cargo/rschat-config.json");
        println!("base_dir after push ：{}", base_dir.to_str().unwrap());

        let target_file = base_dir.clone();
        target_file
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("rschat")
        .version("1.0")
        .author("Your Name")
        .about("A simple chatbot using GPT API")
        .subcommand(
            Command::new("gpt")
                .about("let the chatgpt say something")
                .arg(Arg::new("text").required(true).help("Text to say")),
        )
        .subcommand(
            Command::new("gemini")
                .about("let the gemini say something")
                .arg(Arg::new("text").required(true).help("Text to say")),
        )
        .subcommand(
            Command::new("config")
                .about("Configure the GPT API key")
                .arg(Arg::new("apikey").required(true).help("API key for GPT")),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("gpt", sub_matches)) => {
            let text = sub_matches.get_one::<String>("text").unwrap();
            let config = Config::new()?;
            let api_key = config
                .gpt_api_key
                .ok_or("API key not configured. Run 'rschat config' first.")?;
            let response = handler::chat_gpt_say(text, &api_key)?;
            println!("{}", response);
        }
        Some(("gemini", sub_matches)) => {
            let text = sub_matches.get_one::<String>("text").unwrap();
            let config = Config::new()?;
            let api_key = config
                .gemini_api_key
                .ok_or("API key not configured. Run 'rschat config' first.")?;
            let response = handler::chat_gemini_say(text, &api_key)?;
            println!("{}", response);
        }
        Some(("config", sub_matches)) => {
            let api_key_json = sub_matches.get_one::<String>("apikey").unwrap().to_owned();
            let config: Config = serde_json::from_str(api_key_json.as_str()).unwrap();
            println!("config: {:?}", config);
            config.save()?;
            println!("API key configured successfully.");
        }
        _ => {
            println!("No subcommand provided. Use 'rschat --help' for usage information.");
        }
    }

    Ok(())
}
