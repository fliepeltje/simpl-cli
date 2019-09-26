## Usage

1. Run simpl config or configure your credentials in `$HOMEDIR/.simpl.config.toml` with:
    - api_key = `"{simplicate api key}"`
    - api_secret = `"{simplicate api secret}"`
    - simplicate_host = `"{simplicate host name}"`
    - employee_id = `"{simplicate employee id}"`
2. Add links with aliases for projects you want to book hours to with `simpl links add`
3. Book hours with `simpl book <project name> <time in hours>` optionally provide `-t` arguments to add in the simplicate note, eg `simpl book simpl-cli 0.5 -t book-cmd links-cmd`

Run `simpl --help` to see more detailed commands.

## Build
To install you will need stable Rust. Then in the main directory run `cargo build --release` and move the binary from the target folder to a desired place.