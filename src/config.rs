use colored::*;
use dialoguer::{theme::ColorfulTheme, Input};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use simplicate::Client;
use std::{env, fs};
use structopt::StructOpt;

#[derive(Serialize, Deserialize)]
pub struct SimplicateConfig {
    pub api_key: String,
    pub api_secret: String,
    pub host: String,
    pub employee_id: String,
}

pub fn init_simplicate_client() -> Client {
    Client {
        api_key: std::env::var("SIMPL_API_KEY").expect("No API key in configuration"),
        api_secret: std::env::var("SIMPL_API_SECRET").expect("No API secret in configuration"),
        host: std::env::var("SIMPL_HOST").expect("No host in configuration"),
    }
}

pub fn init_config_env() {
    match UserConfig::from_fs() {
        Some(cfg) => cfg.set_env(),
        None => panic!("No user configuration found. Please run the config command".yellow()),
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserConfig {
    pub simplicate: SimplicateConfig,
}

impl UserConfig {
    pub fn from_fs() -> Option<UserConfig> {
        let simplconf = home_dir()
            .expect("Can't find homedir on this fs")
            .join(".simpl/config.toml");
        match fs::read_to_string(simplconf) {
            Ok(string) => {
                let config: UserConfig = toml::from_str(&string.to_string().to_owned()).unwrap();
                Some(config)
            }
            Err(_) => None,
        }
    }

    fn from_input() -> UserConfig {
        let simplicate = SimplicateConfig {
            api_key: Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your API Key")
                .default(env::var("SIMPL_API_KEY").unwrap_or(String::from("")))
                .interact()
                .unwrap(),
            api_secret: Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your API Secret")
                .default(env::var("SIMPL_API_SECRET").unwrap_or(String::from("")))
                .interact()
                .unwrap(),
            employee_id: Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your employee id")
                .default(env::var("SIMPL_EMPLOYEE_ID").unwrap_or(String::from("")))
                .interact()
                .unwrap(),
            host: Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your simpicate host")
                .default(env::var("SIMPL_HOST").unwrap_or(String::from("")))
                .interact()
                .unwrap(),
        };
        UserConfig {
            simplicate: simplicate,
        }
    }

    fn set_env(&self) {
        env::set_var("SIMPL_API_KEY", &self.simplicate.api_key);
        env::set_var("SIMPL_API_SECRET", &self.simplicate.api_secret);
        env::set_var("SIMPL_HOST", &self.simplicate.host);
        env::set_var("SIMPL_EMPLOYEE_ID", &self.simplicate.employee_id);
    }

    fn store(&self) {
        let homedir = dirs::home_dir().expect("Can't find homedir on this fs");
        let simpldir = homedir.join(".simpl");
        fs::create_dir_all(simpldir).expect("Failed to create simpl dir");
        let simplconf = homedir.join(".simpl/config.toml");
        let toml_string = toml::to_string(self).expect("Could not encode TOML value");
        fs::write(simplconf, toml_string).expect("Failed to write config");
    }
}

impl std::fmt::Display for UserConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line_1 = "Simplicate Config".bold().green();
        let line_2 = format!(
            "{}: {}",
            "api key".italic().green(),
            &self.simplicate.api_key.blue()
        );
        let line_3 = format!("{}: {}", "api secret".italic().green(), "hidden".blue());
        let line_4 = format!(
            "{}: {}",
            "host".italic().green(),
            &self.simplicate.host.blue()
        );
        let line_5 = format!(
            "{}: {}",
            "employee id".italic().green(),
            &self.simplicate.employee_id.blue()
        );
        write!(
            f,
            "{}\n{}\n{}\n{}\n{}",
            line_1, line_2, line_3, line_4, line_5
        )
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "config")]
pub struct ConfigCommand {}

impl ConfigCommand {
    pub fn execute(&self) {
        let current_config = UserConfig::from_fs();
        match current_config {
            Some(cfg) => cfg.set_env(),
            None => println! {"No existing configuration found"},
        };
        let new_config = UserConfig::from_input();
        new_config.store();
        println!("The new configuration is: \n{}", new_config);
    }
}
