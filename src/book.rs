use crate::config::{init_config_env, init_simplicate_client};
use crate::links::Link;
use chrono::offset::Utc;
use colored::*;
use serde::Deserialize;
use serde_json::{to_string_pretty, Value};
use simplicate::structures::NewHours;
use simplicate::Post;
use std::env;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "book")]
pub struct BookCommand {
    /// Project alias
    pub project_alias: String,
    /// Hours spent
    pub time: f64,
    #[structopt(short = "t")]
    /// Relevant tickets (e.g. LABD-001)
    pub tickets: Option<Vec<String>>,
    /// Additional context (e.g. 'took longer due to Amazon issues')
    #[structopt(short = "m")]
    pub context: Option<String>,
}

#[derive(Deserialize)]
struct Response {
    errors: Option<Value>,
}

impl BookCommand {
    pub fn execute(&self) {
        init_config_env();
        let link = Link::from_alias(self.project_alias.to_owned());
        let newhours = NewHours {
            hours: self.time,
            project_id: link.project,
            projectservice_id: link.service,
            employee_id: env::var("SIMPL_EMPLOYEE_ID").expect("No employee ID is set"),
            type_id: link.hourtype,
            start_date: Utc::now().naive_utc(),
            note: Some(self.format_note()),
        };
        let cli = init_simplicate_client();
        let resp: Response = newhours
            .post(cli)
            .expect("Failed to book hours due to an error in the API")
            .json()
            .expect("Failed to parse response");
        match resp.errors {
            None => println!(
                "{}\nHours: {}\nService: {}\nMessage: {}",
                String::from("Booked hours succesfully!").green().bold(),
                newhours.hours.to_string().yellow().italic(),
                link.description.to_string().yellow().italic(),
                newhours
                    .note
                    .unwrap_or("no message".to_string())
                    .yellow()
                    .italic()
            ),
            Some(err) => println!(
                "{}\n\nError Response:\n{}",
                "Failed to book hours due to a configuration error for the given alias, verify that the project is valid".red(),
                format!("{}", to_string_pretty(&err).unwrap_or(String::from("No response"))).yellow().bold()
            ),
        };
    }

    fn format_note(&self) -> String {
        let tickets = match &self.tickets {
            Some(tickets) => {
                let inner = tickets.join("+");
                format!("[{}] ", inner).to_string()
            }
            None => String::from(""),
        };
        let context = self
            .context
            .as_ref()
            .unwrap_or(&String::from(""))
            .to_owned();
        format!("{}{}", tickets, context).to_string()
    }
}
