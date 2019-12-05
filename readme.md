## Usage

1. Run simpl config or configure your credentials in `$HOMEDIR/.simpl/config.toml` with:
```
api_key = "{simplicate api key}"
api_secret = "{simplicate api secret}"
host = "{simplicate host name}"
employee_id = "{simplicate employee id}"
```
Optionally you can provide a `project_status_filter` with a project status ID to filter projects when adding links.
2. Add links with aliases for projects you want to book hours to with `simpl links add`
3. Book hours with `simpl book <project name> <time in hours>` optionally provide `-t` arguments to add one or more  jira tickets and `-m` for additonal context `simpl book myalias 0.5 -t LAB-001 -m "took longer due to dependency updates"`

Run `simpl --help` to see more detailed commands.

## Build
To install you will need stable Rust. Then in the main directory run `cargo build --release` and move the binary from the target folder to a desired place.