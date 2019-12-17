## Usage

1. Run simpl config or configure your credentials in `$HOMEDIR/.simpl/config.toml` with:
```
api_key = "{simplicate api key}"
api_secret = "{simplicate api secret}"
host = "{simplicate host name}"
employee_id = "{simplicate employee id}"
```
2. Add links with aliases for projects you want to book hours to with `simpl links add`
3. Book hours with `simpl book <project name> <time in hours>` optionally you can provide the following arguments: 
 - `-t` to add one or more  jira tickets (`-t LAB-001 LAB-002`)
 - `-m` for additonal context (`-m "ticket took 2 hours longer than planned"`) 
 - `-d` to specify a date for which to book; by default you book for today (`-d "2019-01-01"`)

 So a command with all options would look something like:
 
 `simpl book myalias 0.5 -t LAB-001 -m "took longer due to dependency updates" -d "2019-11-11"`

Run `simpl --help` to see more detailed commands.

## Install
The easiest way to install this CLI is by using `cargo install` with the git flag. The full command is `cargo intall --git=https://github.com/fliepeltje/simpl-cli`

## Build from source
To install you will need stable Rust. Then in the main directory run `cargo build --release` and move the binary from the target folder to a desired place.


