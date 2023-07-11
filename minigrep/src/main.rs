use minigrep_trial_manikya::Config;
use std::{env, process};
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep_trial_manikya::run(config) {
        eprintln!("Could not read file: {e}");
        process::exit(1);
    }
}
