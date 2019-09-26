use crate::links::Link;
use chrono::{NaiveDateTime, TimeZone, Local};
use chrono_tz::Europe::Amsterdam;
use colored::*;
use git2::Repository;
use serde::{Deserialize, Serialize};
use simplicate::hours::HourPost;
use simplicate::generic::CustomField;
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct CommitData {
    pub id: String,
    pub author: String,
    pub message: String,
    pub project: String,
    pub authored_on: String,
}

impl CommitData {
    pub fn latest() -> Option<CommitData> {
        match Repository::open_from_env() {
            Ok(repo) => {
                let commit = repo
                    .head()
                    .expect("no head")
                    .peel_to_commit()
                    .expect("no latest commit");
                let naive_dt = NaiveDateTime::from_timestamp(commit.time().seconds(), 0);
                let mut dir_elements: Vec<String> = repo
                    .workdir()
                    .expect("No dir")
                    .to_str()
                    .unwrap()
                    .to_string()
                    .split("/")
                    .map(|x| x.to_string())
                    .collect();
                let first_element: String = dir_elements
                    .pop()
                    .expect("Invalid directory sturcture")
                    .to_owned();
                let project_name = if first_element != String::from("") {
                    first_element
                } else {
                    dir_elements.pop().expect("No project name").to_owned()
                };
                let data = CommitData {
                    id: commit.id().to_string().to_owned(),
                    author: commit.author().name().expect("No author").to_string(),
                    message: commit.message().expect("No message").to_string(),
                    project: project_name,
                    authored_on: Amsterdam
                        .from_local_datetime(&naive_dt)
                        .unwrap()
                        .to_string(),
                };
                Some(data)
            }
            _ => None,
        }
    }

    pub fn to_customfield(&self) -> CustomField {
        CustomField {
            name: "related_commit".to_string(),
            value: self.id.to_owned(),
            label: Some(self.project.to_owned())
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Loggable {
    pub link: Link,
    pub time: f64,
    pub tags: Vec<String>,
    pub commit: Option<CommitData>,
}

impl Loggable {
    pub fn new(time: String, project_alias: String, add_commit: bool, tags: Vec<String>) -> Loggable {
        let commit_data = match add_commit {
            true => CommitData::latest(),
            false => None,
        };
        let link = Link::get_by_alias(project_alias).expect("No link with specified alias");
        let loggable = Loggable {
            link: link,
            time: time.parse().expect("Time specified is not a float"),
            tags: tags,
            commit: commit_data,
        };
        loggable
    }

    pub fn to_hourpost(&self, employee_id: String) -> HourPost {
        let start = match &self.commit {
            Some(commit) => commit.authored_on.to_owned(),
            None => Local::now().to_string()
        };
        let note = self.tags.join(" | ");
        let postable = HourPost {
            employee_id: employee_id,
            project_id: self.link.project_id.to_owned(),
            projectservice_id: self.link.service_id.to_owned(),
            type_id: self.link.hours_id.to_owned(),
            hours: self.time.to_owned(),
            start_date: start,
            note: note,
            custom_fields: None
        };
        postable
    }
}

impl fmt::Display for Loggable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let project_description = match &self.commit {
            Some(c) => format!(
                "{git}{project}\n{descr}{description}",
                git = "Git Project: ".to_string().bold().bright_red(),
                descr = "Description: ".to_string().bold().green(),
                description = self.link.description,
                project = c.project.to_string()
            ),
            _ => format!(
                "{descr}{description}",
                descr = "Link Description: ".to_string().bold().green(),
                description = self.link.description,
            ),
        };
        let time = format!(
            "{} {} {}",
            "Time:".to_string().bold().purple(),
            self.time.to_string().bold(),
            "hours".to_string()
        );
        let tags = match self.tags.len() {
            0 => "".to_string(),
            _ => format!(
                "\n{} {}",
                "Tags:".to_string().cyan().bold(),
                self.tags.join(" | ").italic()
            ),
        };
        write!(f, "{}\n{} {}", project_description, time, tags)
    }
}
