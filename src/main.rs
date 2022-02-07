use std::env;
use std::process;

//bring `Config` type into scope from library crate
use minigrep::Config;

fn main() {
    //moves args (iterator!) into config new (passes ownership to function)
    //handle result being returned by using a 'closure' (anonymous function)
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);

        process::exit(1);
    });

    //moves ownership of `config` into this function
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
