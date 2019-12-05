mod book;
mod config;
mod links;
mod show;
use structopt::StructOpt;
#[macro_use]
extern crate prettytable;

#[derive(Debug, StructOpt)]
enum Command {
    /// Manage established links
    Links(links::LinkCommand),
    /// Book hours under aliased service
    Book(book::BookCommand),
    /// Create simpl config
    Config(config::ConfigCommand),
    /// Display worked hours
    Show(show::ShowCommand),
}

impl Command {
    fn execute(&self) {
        match self {
            Command::Config(cmd) => cmd.execute(),
            Command::Links(cmd) => cmd.execute(),
            Command::Book(cmd) => cmd.execute(),
            Command::Show(cmd) => cmd.execute(),
        }
    }
}

fn main() {
    let opt = Command::from_args();
    opt.execute();
}
