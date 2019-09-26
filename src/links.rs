use crate::api::{client_from_env, Hourtypes, Links, Projects, Promptable, Services};
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use toml::to_string as to_toml;

#[derive(Serialize, Deserialize, Clone)]
pub struct Link {
    pub alias: String,
    pub project_id: String,
    pub service_id: String,
    pub hours_id: String,
    pub description: String,
}

impl Link {
    pub fn new(project_status_filter: Option<String>) {
        let cli = client_from_env();
        let mut projects: Projects = match project_status_filter {
            Some(filtr) => cli.get_projects_by_status(filtr),
            None => cli.get_projects(),
        };
        projects.sort_by(|b, a| b.name.cmp(&a.name));
        let project_index = projects.index_prompt();
        let project = &projects[project_index];
        let mut services: Services = cli.get_services_by_project(&project.id);
        services.sort_by(|b, a| b.name.cmp(&a.name));
        let service_index = services.index_prompt();
        let service = &services[service_index];
        let mut hourtypes: Hourtypes = cli.get_hourtypes();
        hourtypes.sort_by(|b, a| b.label.cmp(&a.label));
        let hourtype_index = hourtypes.index_prompt();
        let hourtype = &hourtypes[hourtype_index];
        let alias: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter alias")
            .interact()
            .unwrap();
        let service_name = match &service.name {
            Some(x) => x.to_string(),
            None => "default service".to_string(),
        };
        let link = Link {
            alias: alias,
            project_id: project.id.to_string(),
            service_id: service.id.to_string(),
            hours_id: hourtype.id.to_string(),
            description: format!("{} for {}: {}", hourtype.label, project.name, service_name)
                .to_string(),
        };
        link.store();
    }

    pub fn get_links_map() -> HashMap<String, Link> {
        let linksfile = dirs::home_dir()
            .expect("Can't find homedir on this fs")
            .join(".simpl/links.toml");
        let links: HashMap<String, Link> = match fs::read_to_string(&linksfile) {
            Ok(string) => toml::from_str(&string.to_string().to_owned()).unwrap(),
            Err(_) => HashMap::new(),
        };
        links
    }

    pub fn get_links() -> Links {
        let link_map = Link::get_links_map();
        let mut links = vec![];
        for (_, link) in link_map.iter() {
            links.push(link.clone());
        }
        links
    }

    pub fn get_options() -> Vec<String> {
        let links = Link::get_links_map();
        let mut options = vec![];
        for k in links.keys() {
            options.push(k.to_string())
        }
        options
    }

    pub fn store(self) {
        let mut links = Link::get_links_map();
        links.insert(self.alias.to_string(), self);
        let toml_string = to_toml(&links).expect("Couldnt parse links");
        let linksfile = dirs::home_dir()
            .expect("Can't find homedir on this fs")
            .join(".simpl/links.toml");
        fs::write(linksfile, toml_string).expect("Failed to write config");
    }

    pub fn remove(alias: String) {
        let mut links = Link::get_links_map();
        links.remove(&alias);
        let toml_string = to_toml(&links).expect("Couldnt parse links");
        let linksfile = dirs::home_dir()
            .expect("Can't find homedir on this fs")
            .join(".simpl/links.toml");
        fs::write(linksfile, toml_string).expect("Failed to write config");
    }

    pub fn get_by_alias(alias: String) -> Option<Link> {
        let links = Link::get_links_map();
        match links.get(&alias) {
            Some(x) => Some(Link {
                alias: x.alias.to_string(),
                project_id: x.project_id.to_string(),
                service_id: x.service_id.to_string(),
                hours_id: x.hours_id.to_string(),
                description: x.description.to_string(),
            }),
            None => None,
        }
    }
}
