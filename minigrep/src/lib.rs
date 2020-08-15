#![allow(dead_code)]
#![allow(unused_imports)]

use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let haystack = fs::read_to_string(config.filename)?;
    let results = if config.case_insensitive {
        search_case_insensitive(&config.needle, &haystack)
    } else {
        search(&config.needle, &haystack)
    };

    println!("{}", results.join("\n"));

    Ok(())
}

fn search<'a>(needle: &String, haystack: &'a String) -> Vec<&'a str> {
    haystack
        .lines()
        .filter(|line| line.contains(needle))
        .collect::<Vec<_>>()
}

fn search_case_insensitive<'a>(needle: &String, haystack: &'a String) -> Vec<&'a str> {
    let needle = needle.to_lowercase();

    haystack
        .lines()
        .filter(|line| line.to_lowercase().contains(&needle))
        .collect::<Vec<_>>()
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Config {
    pub needle: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next();

        let needle = match args.next() {
            Some(arg) => arg,
            None => return Err("<needle> is required!"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("<filename> is required!"),
        };

        let case_insensitive = env::var("CASE_INSENSITIVE").is_ok();

        Ok(Config {
            needle,
            filename,
            case_insensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_search() {
        let needle = String::from("hello");
        let haystack = String::from("hello world\nholla world\nholla hello");

        assert_eq!(
            search(&needle, &haystack),
            vec!["hello world", "holla hello"]
        );
    }

    #[test]
    fn case_insensitive_search() {
        let needle = String::from("hElLO");
        let haystack = String::from("hello world\nholla world\nholla hello");

        assert_eq!(
            search_case_insensitive(&needle, &haystack),
            vec!["hello world", "holla hello"]
        );
    }
}
