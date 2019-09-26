mod api;
mod book;
mod config;
mod links;
#[macro_use(c)]
extern crate cute;
#[macro_use]
extern crate prettytable;

use clap::{App, Arg, SubCommand};

use crate::api::{Promptable, client_from_env};
use simplicate::traits::Post;
use book::Loggable;
use config::UserConfig;
use links::Link;
use colored::*;

fn main() {
    let links_options = Link::get_options();
    let links_options: Vec<&str> = links_options.iter().map(|x| &**x).collect();
    let user = UserConfig::fetch();
    let user_found = match &user {
        Some(_) => true,
        _ => false,
    };
    if !user_found {
        println!("No user config found, please run config first")
    };
    let matches = App::new("Simpl")
        .version("1.0")
        .author("Donatas Rasiukevicius")
        .about("Log hours to Simplicate")
        .subcommand(
            SubCommand::with_name("links")
                .about("Manage simplicate links")
                .subcommand(SubCommand::with_name("add").about("Add a simplicate link"))
                .subcommand(SubCommand::with_name("show").about("Show current links"))
                .subcommand(
                    SubCommand::with_name("rm").about("Remove a link").arg(
                        Arg::with_name("alias")
                            .index(1)
                            .help("Project alias")
                            .possible_values(&links_options)
                            .required(true),
                    ),
                ),
        )
        .subcommand(SubCommand::with_name("config").about("Configure credentials"))
        .subcommand(SubCommand::with_name("log").about("Show booked hours"))
        .subcommand(
            SubCommand::with_name("book")
                .about("Log hours")
                .arg(
                    Arg::with_name("project")
                        .index(1)
                        .help("Project alias")
                        .possible_values(&links_options)
                        .required(true),
                )
                .arg(
                    Arg::with_name("time")
                        .index(2)
                        .help("Time in hours")
                        .required(true),
                )
                .arg(
                    Arg::with_name("commit")
                        .short("c")
                        .long("commit")
                        .help("Adds latest commit from current repo")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("tags")
                        .short("t")
                        .long("tag")
                        .help("Describe the log")
                        .takes_value(true)
                        .multiple(true),
                ),
        )
        .get_matches();
    match matches.subcommand() {
        ("links", Some(cmd)) => match cmd.subcommand() {
            ("add", _) => {
                Link::new(user.unwrap().project_status_filter);
                Link::get_links().print_table();
            }
            ("show", _) => {
                Link::get_links().print_table();
            }
            ("rm", Some(rm)) => {
                let alias = rm.value_of("alias").expect("No alias specified").to_owned();
                Link::remove(alias.clone());
                println!("Removed alias {}", alias);
                Link::get_links().print_table();
            }
            _ => panic!("Unknown Command"),
        },
        ("book", Some(cmd)) => {
            let loggable = Loggable::new(
                cmd.value_of("time").unwrap().to_owned(),
                cmd.value_of("project").unwrap().to_owned(),
                cmd.is_present("commit"),
                match cmd.values_of("tags") {
                    Some(vals) => c![x.to_string().to_owned(), for x in vals],
                    _ => Vec::new(),
                },
            );
            let postable = loggable.to_hourpost(user.unwrap().employee_id.to_owned());
            let cli = client_from_env();
            postable.post(cli);
            println!("{}", "Succesfully posted hours to simplicate!".to_string().bold().green());
        }
        ("config", Some(_)) => UserConfig::set_from_cli(),
        _ => panic!("Unknown Command"),
    }
}
