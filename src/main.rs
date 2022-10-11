use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let conf = Config::build(&args).unwrap_or_else(|err| {
        println!("error parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(conf) {
        println!("application error: {e}");
        process::exit(1);
    }
}
