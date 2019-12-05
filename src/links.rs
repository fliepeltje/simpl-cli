use crate::config::{init_config_env, init_simplicate_client};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input};
use prettytable::{Row, Table};
use serde::{Deserialize, Serialize};
use simplicate::structures::{HourType, Project, Service};
use simplicate::QueryMany;
use std::collections::HashMap;
use std::fs;
use structopt::StructOpt;
use toml;

trait Promptable<T: QueryMany<T>> {
    const PROMPT_TEXT: &'static str;
    fn format_row(item: &T, index: usize) -> Row;
    fn format_headers() -> Row;

    fn extra_params() -> Option<Vec<(String, String)>> {
        None
    }

    fn sort(items: Vec<T>) -> Vec<T> {
        items
    }

    fn filter(items: Vec<T>) -> Vec<T> {
        items
    }

    fn retrieve(offset: Option<u32>) -> Vec<T> {
        let offset = match offset {
            Some(x) => x,
            None => 0,
        };
        let mut params = vec![
            ("limit".to_string(), "100".to_string()),
            ("offset".to_string(), offset.to_string()),
        ];
        match Self::extra_params() {
            Some(p) => {
                params.extend(p);
            }
            None => (),
        };
        let items = T::fetch_many(init_simplicate_client(), Some(params));
        match items {
            Some(mut list) => {
                if list.len() == 100 {
                    let new_offset = offset + 100;
                    let next = Self::retrieve(Some(new_offset));
                    list.extend(next);
                    list
                } else {
                    list
                }
            }
            None => panic!("fail"),
        }
    }

    fn print_options(items: &Vec<T>) {
        let mut table = Table::new();
        table.add_row(Self::format_headers());
        for (index, item) in items.iter().enumerate() {
            table.add_row(Self::format_row(item, index));
        }
        table.printstd();
    }

    fn prompt() -> T {
        let items = Self::retrieve(None);
        let items = Self::sort(items);
        let items = Self::filter(items);
        Self::print_options(&items);
        let selection: usize = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(Self::PROMPT_TEXT)
            .interact()
            .unwrap();
        items.into_iter().nth(selection).expect("Invalid Selection")
    }
}

impl Promptable<Project> for Link {
    const PROMPT_TEXT: &'static str = "Select project index";

    fn sort(mut items: Vec<Project>) -> Vec<Project> {
        items.sort_by_key(|x| {
            (
                match &x.project_status {
                    Some(x) => x.label.to_string(),
                    None => "Unknown".to_string(),
                },
                x.name.to_string(),
            )
        });
        items
    }

    fn format_headers() -> Row {
        row![
            "Index".bold().yellow(),
            "Project Name".bold().yellow(),
            "Start Date".bold().yellow(),
            "End Date".bold().yellow(),
            "Status".bold().yellow()
        ]
    }

    fn format_row(item: &Project, index: usize) -> Row {
        let status = match &item.project_status {
            Some(status) => &status.label,
            None => "Unknown",
        };
        let active = status == "tab_pactive";
        let start_date = match &item.start_date {
            Some(date) => date.to_string(),
            None => "Unknown".to_string(),
        };
        let end_date = match &item.end_date {
            Some(date) => date.to_string(),
            None => "Unknown".to_string(),
        };
        match active {
            true => row![
                index.to_string().bold(),
                item.name.green(),
                start_date.green().italic(),
                end_date.green().italic(),
                status.green().italic()
            ],
            false => row![
                index.to_string().bold(),
                item.name.red(),
                start_date.red().italic(),
                end_date.red().italic(),
                status.red().italic()
            ],
        }
    }
}

impl Promptable<Service> for Link {
    const PROMPT_TEXT: &'static str = "Select service index";

    fn sort(mut items: Vec<Service>) -> Vec<Service> {
        items.sort_by_key(|x| (x.name.as_ref().unwrap_or(&"".to_string()).to_owned()));
        items
    }

    fn format_headers() -> Row {
        row![
            "Index".bold().yellow(),
            "Service Name".bold().yellow(),
            "Start Date".bold().yellow(),
            "End Date".bold().yellow(),
            "Status".bold().yellow()
        ]
    }

    fn extra_params() -> Option<Vec<(String, String)>> {
        let project_id = std::env::var("PROJECT_ID").expect("No project ID set");
        Some(vec![("q[project_id]".to_string(), project_id.to_string())])
    }

    fn format_row(item: &Service, index: usize) -> Row {
        let status = match &item.status {
            Some(status) => &status,
            None => "Unknown",
        };
        let active = status == "open";
        let start_date = match &item.start_date {
            Some(datetime) => datetime.to_string(),
            None => "Unknown".to_string(),
        };
        let end_date = match &item.end_date {
            Some(datetime) => datetime.to_string(),
            None => "Unknown".to_string(),
        };
        let name = match &item.name {
            Some(name) => name,
            None => "Unknown",
        };
        match active {
            true => row![
                index.to_string().bold(),
                name.green(),
                start_date.green().italic(),
                end_date.green().italic(),
                status.green().italic()
            ],
            false => row![
                index.to_string().bold(),
                name.red(),
                start_date.red().italic(),
                end_date.red().italic(),
                status.red().italic()
            ],
        }
    }
}

impl Promptable<HourType> for Link {
    const PROMPT_TEXT: &'static str = "Select Hourtype index";

    fn sort(mut items: Vec<HourType>) -> Vec<HourType> {
        items.sort_by_key(|x| (x.label.to_string()));
        items
    }

    fn format_headers() -> Row {
        row!["Index".bold().yellow(), "Name".bold().yellow(),]
    }

    fn format_row(item: &HourType, index: usize) -> Row {
        row![index.to_string().bold(), item.label.green()]
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Link {
    pub alias: String,
    pub project: String,
    pub service: String,
    pub hourtype: String,
    pub description: String,
}

impl Link {
    fn save(self) {
        let mut links = Link::get_mapping();
        links.insert(self.alias.to_string(), self);
        let toml_string = toml::to_string(&links).expect("Couldnt parse links");
        let linksfile = dirs::home_dir()
            .expect("Can't find homedir on this fs")
            .join(".simpl/links.toml");
        fs::write(linksfile, toml_string).expect("Failed to write config");
    }

    fn remove(alias: String) {
        let mut links = Link::get_mapping();
        links.remove(&alias);
        let toml_string = toml::to_string(&links).expect("Couldnt parse links");
        let linksfile = dirs::home_dir()
            .expect("Can't find homedir on this fs")
            .join(".simpl/links.toml");
        fs::write(linksfile, toml_string).expect("Failed to write config");
    }

    fn from_prompt() -> Link {
        let project: Project = Self::prompt();
        std::env::set_var("PROJECT_ID", &project.id);
        let service: Service = Self::prompt();
        let hourtype: HourType = Self::prompt();
        let description = format!(
            "{} for {} - {}",
            &hourtype.label,
            &project.name,
            &service.name.unwrap_or("Unnamed Service".to_string())
        );
        Link {
            project: project.id,
            service: service.id,
            hourtype: hourtype.id,
            alias: Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter an alias")
                .interact()
                .unwrap(),
            description,
        }
    }

    pub fn from_alias(alias: String) -> Link {
        let links = Self::get_mapping();
        match links.get(&alias) {
            Some(x) => Link {
                alias: x.alias.to_string(),
                project: x.project.to_string(),
                service: x.service.to_string(),
                hourtype: x.hourtype.to_string(),
                description: x.description.to_string(),
            },
            None => panic!("No link for alias"),
        }
    }

    fn get_links() -> Links {
        let link_map = Self::get_mapping();
        let mut links = vec![];
        for (_, link) in link_map.iter() {
            links.push(link.clone());
        }
        links
    }

    fn get_mapping() -> HashMap<String, Link> {
        let linksfile = dirs::home_dir()
            .expect("Can't find home directory on this filesystem")
            .join(".simpl/links.toml");
        let links: HashMap<String, Link> = match fs::read_to_string(&linksfile) {
            Ok(string) => toml::from_str(&string.to_string().to_owned()).unwrap(),
            Err(_) => HashMap::new(),
        };
        links
    }
}

type Links = Vec<Link>;

trait TableDisplay<T> {
    fn format_row(item: &T, index: usize) -> Row;
    fn format_headers() -> Row;

    fn sort(items: Vec<T>) -> Vec<T> {
        items
    }

    fn filter(items: Vec<T>) -> Vec<T> {
        items
    }

    fn print_table(items: &Vec<T>) {
        let mut table = Table::new();
        table.add_row(Self::format_headers());
        for (index, item) in items.iter().enumerate() {
            table.add_row(Self::format_row(item, index));
        }
        table.printstd();
    }
}

impl TableDisplay<Link> for Links {
    fn format_headers() -> Row {
        row![
            "Index".bold().yellow(),
            "Alias".bold().yellow(),
            "Description".bold().yellow()
        ]
    }

    fn format_row(item: &Link, index: usize) -> Row {
        row![
            index.to_string().bold(),
            item.alias.green().bold(),
            item.description.green()
        ]
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "links")]
pub enum LinkCommand {
    /// Add new link from all projects
    #[structopt(name = "add")]
    Add,
    /// Remove an existing link by alias
    #[structopt(name = "rm")]
    Remove { alias: String },
    /// Show the existing links
    #[structopt(name = "show")]
    Show,
}

impl LinkCommand {
    pub fn execute(&self) {
        init_config_env();
        match self {
            LinkCommand::Add => {
                let new_link = Link::from_prompt();
                let description = &new_link.description.to_owned();
                let alias = &new_link.alias.to_owned();
                new_link.save();
                println!(
                    "Succesfully added link alias {} for {}",
                    alias.bold().green(),
                    description.green()
                );
            }
            LinkCommand::Remove { alias } => {
                Link::remove(alias.to_string());
                println!("Succesfully removed link alias {}", alias.green());
            }
            LinkCommand::Show => {
                let links: Links = Link::get_links();
                Links::print_table(&links);
            }
        }
    }
}
