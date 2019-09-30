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
    pub fn retrieve_current_week(employee_id: String) -> HoursLog {
        let current_dt: NaiveDate = Local::today().naive_local();
        let y = &current_dt.iso_week().year();
        let w = &current_dt.iso_week().week();
        let start_date = NaiveDate::from_isoywd(*y, *w, Weekday::Mon);
        let end_date = NaiveDate::from_isoywd(*y, *w, Weekday::Sat);
        let cli = client_from_env();
        HoursLog {
            hours: cli.get_employee_hours_for_daterange(
                employee_id,
                Some(start_date.to_string()),
                Some(end_date.to_string()),
            ),
        }
    }

    pub fn print(self) {
        let mut hours = self.hours;
        hours.sort_by(|a, b| a.start_date.cmp(&b.start_date));
        let mut header = "unknown".to_string();
        for h in hours {
            match &h.start_date {
                Some(x) => {
                    let h = x.split(" ").next().unwrap_or("unknown");
                    let h = h.to_string();
                    if h != header {
                        header = h;
                        println!("{}", header.bold().green());
                    };
                }
                None => (),
            };
            let lh = LoggedHour {
                description: match h.project {
                    Some(proj) => {
                        let name = proj.name.unwrap_or("Unnamed project".to_string());
                        let note = h.note.unwrap_or("".to_string());
                        format!("{}: {}", name.bright_red(), note.yellow())
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
            println!(
                "    {}\t\t{}\t{}",
                lh.updated_at.italic().magenta(),
                lh.time.to_string().bold().blue(),
                lh.description,
            );
        }
    }
}
