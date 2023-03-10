use std::error::Error;
use std::{env, fs};

/// the configuration struct
/// for the search process
pub struct Config {
    /// the query to search for
    pub query: String,
    /// the file path to search its content
    pub file_path: String,
    /// determines  the search process case sensitivity
    pub ignore_case: bool,
}

impl Config {
    /// builds the search config object
    /// # Example
    /// ```
    /// let args = vec![String::from("cwf"), String::from("query"), String::from("filetosearch")];
    /// let config = minigrep::Config::build(args.into_iter()).unwrap();
    /// ```
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        //  the first value in the return value of env::args is the name of the program.
        // We want to ignore that and get to the next value,
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

/// Runs the search process using the config argument provided
///
/// # Examples
/// ```
/// let args = vec![String::from("cwf"), String::from("query"), String::from("filetosearch")];
/// let config = minigrep::Config::build(args.into_iter()).unwrap();
///  minigrep::run(config);
/// ```
///
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

fn search<'a, 'b>(query: &str, content: &'a str) -> Vec<&'a str> {
    return content
        .lines()
        .filter(|line| line.contains(query))
        .collect();
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    return content
        .lines()
        .filter(|line| line.contains(&query))
        .collect();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
