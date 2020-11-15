pub mod errors;

use chrono::Local;
use clap::ArgMatches;
use errors::{Error, Result};
use rand::{distributions::Alphanumeric, Rng};
use std::fs::File;
use std::process::{Command, Stdio};

pub fn run(matches: ArgMatches) -> Result<()> {
    let filename = build_filename(
        matches.value_of("prefix"),
        matches
            .value_of("extension")
            .ok_or(Error::OptionParsingFailed("extension".to_string()))?,
    )?;

    File::create(&filename).map_err(|_| Error::FileCreationFailed(filename.clone()))?;

    if matches.is_present("open") {
        Command::new(
            matches
                .value_of("editor")
                .ok_or(Error::OptionParsingFailed("extension".to_string()))?,
        )
        .arg(&filename)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("failed to edit file");
    }

    Ok(())
}

fn generage_random_chars(nchars: usize) -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(nchars)
        .collect::<String>()
}

fn build_filename(prefix: Option<&str>, extension: &str) -> Result<String> {
    let now = Local::now().format("%Y_%m_%d_%H_%M_%S").to_string();
    let rand_chars = generage_random_chars(8);

    if extension.len() == 0 {
        return Err(Error::EmptyStringArgument("extension".to_string()));
    }

    match prefix {
        Some(prefix) if prefix.len() == 0 => {
            return Err(Error::EmptyStringArgument("prefix".to_string()));
        }
        Some(prefix) => Ok(format!("{}_{}_{}.{}", prefix, now, rand_chars, extension)),
        None => Ok(format!("{}_{}.{}", now, rand_chars, extension)),
    }
}

#[cfg(test)]
mod tests {
    use super::build_filename;
    use super::generage_random_chars;
    use super::Error;

    #[test]
    fn test_generated_random_chars_have_correct_length() {
        for n in 1..=10 {
            assert_eq!(generage_random_chars(n).len(), n);
        }
    }

    #[test]
    fn test_built_filname_has_correct_prefix_and_extension() {
        let cases = vec![("prefix1", "txt"), ("prefix2", "md")];
        for (prefix, extension) in &cases {
            match build_filename(Some(prefix), extension) {
                Ok(filename) => {
                    assert!(filename.starts_with(prefix));
                    assert!(filename.ends_with(extension));
                }
                Err(_) => assert!(false),
            }
        }
    }

    #[test]
    fn test_prefix_and_extension_cannot_be_empty_string() {
        let cases = vec![("", "txt"), ("prefix", "")];
        for (prefix, extension) in &cases {
            match build_filename(Some(prefix), extension) {
                Ok(_) => assert!(false),
                Err(Error::EmptyStringArgument(_)) => assert!(true),
                Err(_) => assert!(false),
            }
        }
    }

    #[test]
    fn test_prefix_can_be_none() {
        match build_filename(None, "md") {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
}
