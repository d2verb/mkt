use clap::{App, Arg};
use dirs::home_dir;
use mkt::errors::Result;
use mkt::execute;
use mkt::options;

fn run() -> Result<()> {
    let config_path = home_dir().unwrap().join(".mktrc");
    let mut opts = options::parse(config_path.to_str().unwrap())?;

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
                .takes_value(true),
        )
        .arg(
            Arg::with_name("editor")
                .help("Editor to edit temporary note")
                .short("e")
                .long("editor")
                .takes_value(true),
        )
        .get_matches();

    if matches.is_present("open") {
        opts.open = true
    }

    if matches.is_present("prefix") {
        opts.prefix = matches.value_of("prefix").map(|s| s.to_string());
    }

    if matches.is_present("extension") {
        opts.extension = matches.value_of("extension").unwrap().to_string();
    }

    if matches.is_present("editor") {
        opts.editor = matches.value_of("editor").unwrap().to_string();
    }

    execute(opts)
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}
