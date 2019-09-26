use serde::{Deserialize, Serialize};
use std::{env, fs};
use dialoguer::{theme::ColorfulTheme, Input};
use toml::to_string as to_toml;

#[derive(Serialize, Deserialize)]
pub struct UserConfig {
    pub api_key: String,
    pub api_secret: String,
    pub employee_id: String,
    pub simplicate_host: String,
}

impl UserConfig {
    pub fn set_from_cli() {
        let config = UserConfig {
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
            simplicate_host: Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your simpicate host")
                .default(env::var("SIMPL_HOST").unwrap_or(String::from("")))
                .interact()
                .unwrap(),
        };
        config.store();
    }

    pub fn fetch() -> Option<UserConfig> {
        let simplconf = dirs::home_dir()
            .expect("Can't find homedir on this fs")
            .join("./simpl/config.toml");
        match fs::read_to_string(simplconf) {
            Ok(string) => {
                let config: UserConfig = toml::from_str(&string.to_string().to_owned()).unwrap();
                env::set_var("SIMPL_API_KEY", &config.api_key);
                env::set_var("SIMPL_API_SECRET", &config.api_secret);
                env::set_var("SIMPL_HOST", &config.simplicate_host);
                env::set_var("SIMPL_EMPLOYEE_ID", &config.employee_id);
                Some(config)
            }
            Err(_) => None,
        }
    }

    pub fn store(self) {
        let homedir = dirs::home_dir().expect("Can't find homedir on this fs");
        let simpldir = homedir.join("./simpl");
        fs::create_dir_all(simpldir).expect("Failed to create simpl dir");
        let simplconf = homedir.join("./simpl/config.toml");
        let toml_string = to_toml(&self).expect("Could not encode TOML value");
        println!("config:\n{}", toml_string);
        fs::write(simplconf, toml_string).expect("Failed to write config");
    }
}