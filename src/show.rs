use crate::config::{init_config_env, init_simplicate_client};
use chrono::{Datelike, NaiveDate, Utc, Weekday};
use colored::*;
use simplicate::structures::Hours;
use simplicate::QueryMany;
use std::env;
use structopt::StructOpt;
use chrono::NaiveDateTime;

#[derive(Debug, StructOpt)]
#[structopt(name = "show")]
pub struct ShowCommand {
    /// Specify a start date time from when you want to view the hours (YYYY-MM-DDTHH-MM:SS)
    /// defaults to last monday
    #[structopt(short = "s")]
    pub start_time: Option<NaiveDateTime>,

    /// Specify a end date time till when you want to view the hours (YYYY-MM-DDTHH-MM:SS)
    /// defaults to next saturday
    #[structopt(short = "e")]
    pub end_time: Option<NaiveDateTime>,
}

impl ShowCommand {
    pub fn execute(&self) {
        init_config_env();
        let cli = init_simplicate_client();
        let current_dt: NaiveDate = Utc::today().naive_local();
        let y = &current_dt.iso_week().year();
        let w = &current_dt.iso_week().week();
        let start_date = match self.start_time {
            Some(dt) => dt,
            None => NaiveDate::from_isoywd(*y, *w, Weekday::Mon).and_hms(0, 0, 0),
        };
        let end_date = match self.end_time {
            Some(dt) => dt,
            None => NaiveDate::from_isoywd(*y, *w, Weekday::Sat).and_hms(23, 59, 59),
        };
        let params = vec![
            (
                String::from("q[employee.id]"),
                env::var("SIMPL_EMPLOYEE_ID")
                    .expect("No employee ID set")
                    .to_string(),
            ),
            (String::from("q[start_date][ge]"), start_date.to_string()),
            (String::from("q[start_date][le]"), end_date.to_string()),
        ];
        let mut hours: Vec<Hours> = Hours::fetch_many(cli, Some(params)).expect("No hours found");
        hours.sort_by(|a, b| a.start_date.cmp(&b.start_date));
        let mut header = "unknown".to_string();
        let mut total: Vec<f64> = vec![];
        for h in hours {
            match &h.start_date {
                Some(x) => {
                    let h = x.split(" ").next().unwrap_or("unknown");
                    let h = h.to_string();
                    if h != header {
                        if total.len() > 1 {
                            println!("    -----------------------");
                            let h_total: f64 = total.iter().sum();
                            println!(
                                "    {}\t\t{}\n",
                                String::from("Total").bold().magenta(),
                                h_total.to_string().bold().green(),
                            );
                        }
                        header = h;
                        total = vec![];
                        println!("{}\n", header.bold().green());
                    };
                }
                None => (),
            };
            let proj_name = match h.project {
                Some(p) => p.name.unwrap_or("Unnamed project".to_string()),
                None => "Unnamed project".to_string(),
            };
            let serv_name = match h.projectservice {
                Some(s) => {
                    format!(" / {}", s.name.unwrap_or("Unnamed Service".to_string())).to_string()
                }
                None => String::from(""),
            };
            let note = match h.note {
                Some(n) => {
                    if n == String::from("") {
                        n
                    } else {
                        format!(": {}", n).to_string()
                    }
                }
                None => String::from(""),
            };
            let lh = LoggedHour {
                description: format!(
                    "{}{}{}",
                    proj_name.bright_red(),
                    serv_name.red(),
                    note.yellow()
                )
                .to_string(),
                time: ((h.hours * 100.0).round()) / 100.0,
                updated_at: match h.start_date {
                    Some(s) => {
                        let mut elements = s.split(" ");
                        elements.next();
                        elements
                            .next()
                            .unwrap_or(&"no time".to_string())
                            .to_string()
                    }
                    _ => "unknown".to_string(),
                },
            };
            total.push(lh.time);
            println!(
                "    {}\t\t{}\t{}",
                lh.updated_at.italic().magenta(),
                lh.time.to_string().bold().italic().green(),
                lh.description,
            );
        }
        if total.len() > 1 {
            println!("    -----------------------");
            let h_total: f64 = total.iter().sum();
            println!(
                "    {}\t\t{}\n",
                String::from("Total").bold().magenta(),
                h_total.to_string().bold().green(),
            );
        };
    }
}

#[derive(Clone)]
pub struct LoggedHour {
    pub description: String,
    pub time: f64,
    pub updated_at: String,
}
