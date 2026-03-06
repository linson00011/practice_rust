use anyhow;
use anyhow::Result;
use std::{env, fs};
mod utils;

#[derive(Debug)]
pub struct Config {
    _query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, String> {
        args.next();
        let mut query: String = String::new();
        let mut filename: String = String::new();
        let mut b_case = true; //default value is true. it is sensitive.
        let mut error_string: String = String::new();

        match args.next() {
            Some(arg) => {
                filename = arg;
            }
            None => {
                error_string.push_str("Didn't get a file string.");
            }
        };

        match args.next() {
            Some(arg) => {
                query = arg;
            }
            None => {
                error_string.push_str("Didn't get a query string.");
            }
        };

        let case_sensitive = env::var("CASE").unwrap_or_else(|_e| "1".to_string());
        if case_sensitive == "0" {
            b_case = false;
        }

        if !query.is_empty() && !filename.is_empty() {
            Ok(Config {
                _query: query.to_string(),
                filename: filename.to_string(),
                case_sensitive: b_case,
            })
        } else {
            Err(error_string)
        }
    }
}

pub fn run(config: &Config) -> Result<()> {
    let mut contents = fs::read_to_string(&config.filename)?;
    let mut query = config._query.clone();

    if !config.case_sensitive {
        contents = contents.to_lowercase();
        query = query.to_lowercase();
    }

    let result_filter = search(&contents, &query);

    //let get_owership=contents; can't get owership,because it is borrowed,and lifetime need retain to next line with `result_filter`.

    printlns_simple!(contents, result_filter);

    let get_owership = contents;

    Ok(())
}

pub fn search<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
    //1.split or get lins 2.loop and filter 3. return
    let lines = contents.lines();
    let mut res: Vec<&str> = Vec::new();

    if !query.is_empty() {
        lines.filter(|x| (*x).contains(query)).for_each(|x| {
            res.push(x);
        });
    }

    res
}

mod tests {

    #[test]
    fn temp_test() {
        let mut lines: Vec<String> = Vec::new();
        lines.push("abc".to_string());
        lines.push("aefg".to_string());

        let bb: Vec<&str> = lines
            .iter()
            .filter(|x| (*x).contains(""))
            .map(|x| x.as_str())
            .collect();

        print!("a");
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let conents = "Rust:safe,\nfast, productive.\nPick three.";

        assert_eq!(
            crate::search(conents, query),
            vec!["fast, productive.".to_string()]
        );

        let query = "ducta";
        assert_eq!(crate::search(conents, query), Vec::<String>::new());

        let query = "";
        assert_eq!(crate::search(conents, query), Vec::<String>::new());

        let query = "duct";
        let conents = "";
        assert_eq!(crate::search(conents, query), Vec::<String>::new());

        let query = "";
        let conents = "";
        assert_eq!(crate::search(conents, query), Vec::<String>::new());
    }

    #[test]
    fn one_result_case_sensitive() {
        let query = "Duct";
        let conents = "Rust:safe,\nfast, productive.\nPick three.";

        let squery = &query.to_lowercase();
        let sconents = &conents.to_lowercase();

        assert_eq!(
            crate::search(sconents, squery),
            vec!["fast, productive.".to_string()]
        );

        assert_eq!(crate::search(conents, query), Vec::<String>::new());
    }
}
