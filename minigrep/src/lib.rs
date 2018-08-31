use std::fs;
use std::env;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(&config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &file_contents)
    } else {
        search_case_insensitive(&config.query, &file_contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())   
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matching_lines = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            matching_lines.push(line);
        }
    }

    matching_lines
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut matching_lines = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            matching_lines.push(line);
        }
    }

    matching_lines
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_result() {
        let query = "haiku";
        let contents = "\
This is multi line
It won't have a certain word
So that this test works.";
        let expected: Vec<&str> = Vec::new();

        assert_eq!(
            expected,
            search(&query, &contents)
        );
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
Safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["Safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn three_result() {
        let query = "body";
        let contents = "\
I’m nobody! Who are you?
Are you nobody, too?
Then there’s a pair of us — don’t tell!
They’d banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog.";

        assert_eq!(
            vec![
                "I’m nobody! Who are you?",
                "Are you nobody, too?",
                "How dreary to be somebody!"
            ], search(&query, &contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
Safe, fast, productive.
Pick three.
Duct tape, by the way.";

        assert_eq!(
            vec!["Safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
Safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}