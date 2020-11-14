use clap::{App, Arg};
use mkt::run;

fn main() {
    let matches = App::new("mkt")
        .version("0.1.0")
        .about("A tool to make a temporary note")
        .arg(
            Arg::with_name("edit")
                .help("Opens editor to edit note")
                .short("e")
                .long("edit"),
        )
        .arg(
            Arg::with_name("prefix")
                .help("Prefix of temporary note")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    std::process::exit(match run(matches) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}
