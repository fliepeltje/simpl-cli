use crate::api::client_from_env;
use chrono::{Datelike, Local, NaiveDate, Weekday};
use colored::*;
use simplicate::hours::Hours;

#[derive(Clone)]
pub struct LoggedHour {
    pub description: String,
    pub time: f64,
    pub updated_at: String,
}

pub struct HoursLog {
    pub hours: Vec<Hours>,
}

impl HoursLog {
    pub fn show_current_week(employee_id: String) {
        let current_dt: NaiveDate = Local::today().naive_local();
        let y = &current_dt.iso_week().year();
        let w = &current_dt.iso_week().week();
        let start_date = NaiveDate::from_isoywd(*y, *w, Weekday::Mon);
        let end_date = NaiveDate::from_isoywd(*y, *w, Weekday::Sat);
        let cli = client_from_env();
        let log = HoursLog {
            hours: cli.get_employee_hours_for_daterange(
                employee_id,
                Some(start_date.to_string()),
                Some(end_date.to_string()),
            ),
        };
        log.print();
    }

    pub fn show_latest(employee_id: String) {
        let current_dt: NaiveDate = Local::today().naive_local();
        let dt_string = current_dt.to_string();
        let cli = client_from_env();
        let hours = cli.get_latest_employee_hours_for_date(employee_id, dt_string.clone());
        match hours {
            Some(h) => {
                let log = HoursLog { hours: vec![h] };
                log.print()
            }
            None => println!("No latest hours for {}", dt_string),
        };
    }

    pub fn print(self) {
        let mut hours = self.hours;
        hours.sort_by(|a, b| a.start_date.cmp(&b.start_date));
        let mut header = "unknown".to_string();
        let mut total: f64 = 0.0;
        for h in hours {
            match &h.start_date {
                Some(x) => {
                    let h = x.split(" ").next().unwrap_or("unknown");
                    let h = h.to_string();
                    if h != header {
                        if total > 0.0 {
                            println!("    -----------------------");
                            println!(
                                "    {}\t\t{}\n",
                                String::from("Total").bold().magenta(),
                                total.to_string().bold().green(),
                            );
                        }
                        header = h;
                        total = 0.0;
                        println!("{}\n", header.bold().green());
                    };
                }
                None => (),
            };
            let lh = LoggedHour {
                description: match h.project {
                    Some(proj) => {
                        let name = proj.name.unwrap_or("Unnamed project".to_string());
                        let note = h.note.unwrap_or("".to_string());
                        match note == String::from("") {
                            false => format!("{}: {}", name.bright_red(), note.yellow()),
                            true => format!("{}", name.bright_red()),
                        }
                    }
                    None => "Unknown project".to_string(),
                },
                time: h.hours,
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
            total += lh.time;
            println!(
                "    {}\t\t{}\t{}",
                lh.updated_at.italic().magenta(),
                lh.time.to_string().bold().italic().green(),
                lh.description,
            );
        }
    }
}
