pub mod errors;

use chrono::Local;
use clap::ArgMatches;
use errors::{Error, ErrorKind, Result};
use rand::{distributions::Alphanumeric, Rng};
use std::fs::File;
use std::process::{Command, Stdio};

pub fn run(matches: ArgMatches) -> Result<()> {
    let filename = match build_filename(matches.value_of("prefix"), "md") {
        Ok(filename) => filename,
        Err(err) => return Err(err),
    };

    match File::create(&filename) {
        Ok(_) => {}
        Err(err) => {
            return Err(Error::new(
                ErrorKind::FileCreationFailed,
                Some(format!("{}", err).as_str()),
            ))
        }
    }

    if matches.is_present("edit") {
        Command::new("vim")
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
        return Err(Error::new(
            ErrorKind::InvalidArgument,
            Some("extension is empty string"),
        ));
    }

    match prefix {
        Some(prefix) if prefix.len() == 0 => {
            return Err(Error::new(
                ErrorKind::InvalidArgument,
                Some("prefix is empty string"),
            ));
        }
        Some(prefix) => Ok(format!("{}_{}_{}.{}", prefix, now, rand_chars, extension)),
        None => Ok(format!("{}_{}.{}", now, rand_chars, extension)),
    }
}

#[cfg(test)]
mod tests {
    use super::build_filename;
    use super::generage_random_chars;
    use super::ErrorKind;

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
                Err(err) => assert_eq!(err.kind, ErrorKind::InvalidArgument),
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
