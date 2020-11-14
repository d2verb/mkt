use clap::{App, Arg};
use mkt::run;

fn main() {
    let matches = App::new("mkt")
        .version("0.1.0")
        .about("A tool to make a temporary note")
        .arg(
            Arg::with_name("open")
                .help("Opens editor to edit note")
                .short("o")
                .long("open"),
        )
        .arg(
            Arg::with_name("prefix")
                .help("Prefix of temporary note")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("extension")
                .help("Extension of temporary note")
                .short("x")
                .long("extension")
                .takes_value(true)
                .default_value("md"),
        )
        .arg(
            Arg::with_name("editor")
                .help("Editor to edit temporary note")
                .short("e")
                .long("editor")
                .takes_value(true)
                .default_value("vim"),
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
