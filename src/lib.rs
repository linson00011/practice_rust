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
  pub  fn new(mut args:std::env::Args) -> Result<Config,String> {

        let mut query:String=String::new();
        let mut filename:String=String::new();
        let mut b_case = true; //default value is true. it is sensitive.
        let mut error_string:String=String::new();

        match args.next() {
            Some(arg)=>{
                query=arg;
            },
            None=>{
                error_string.push_str("Didn't get a query string.");
            }
        };

        match args.next() {
            Some(arg)=>{
                filename=arg;
            },
            None=>{
                error_string.push_str("Didn't get a file string.");
            }
        };

        let case_sensitive = env::var("CASE").unwrap_or_else(|_e| "1".to_string());
        if case_sensitive == "0" {
            b_case = false;
        }

        if !query.is_empty() && !filename.is_empty(){
            Ok(
                Config {
                _query: query.to_string(),
                filename: filename.to_string(),
                case_sensitive: b_case,
                }
            )
        }
        else {
            Err(error_string)
        }
    }
}


pub fn run(config: &Config) -> Result<()> {
    let contents = fs::read_to_string(&config.filename)?;
    let result_filter = search_case_sensitive(&contents, &config._query, config.case_sensitive);
    printlns_simple!(contents, result_filter);

    Ok(())
}

pub fn search_case_sensitive(contents: &str, query: &str, case_sensitive: bool) -> Vec<String> {
    //1.sensitive->search 2.insentive-> lowercase->seach.
    if case_sensitive {
        search(contents, query)
    } else {
        let contents_lowercase = contents.to_lowercase();
        let query_lowercase = query.to_lowercase();
        search(&contents_lowercase, &query_lowercase)
    }
}

pub fn search(contents: &str, query: &str) -> Vec<String> {
    //1.split or get lins 2.loop and filter 3. return
    let lines = contents.lines();
    let mut res: Vec<String> = Vec::new();
    if !query.trim().is_empty() && !contents.trim().is_empty() {
        for the_line in lines {
            if the_line.contains(query) {
                res.push(the_line.to_string());
            }
        }
    }

    res
}

mod tests {

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

        assert_eq!(
            crate::search_case_sensitive(conents, query, false),
            vec!["fast, productive.".to_string()]
        );

        assert_eq!(
            crate::search_case_sensitive(conents, query, true),
            Vec::<String>::new()
        );

        let query = "ducta";
        assert_eq!(
            crate::search_case_sensitive(conents, query, false),
            Vec::<String>::new()
        );

        let query = "";
        assert_eq!(
            crate::search_case_sensitive(conents, query, false),
            Vec::<String>::new()
        );

        let query = "duct";
        let conents = "";
        assert_eq!(
            crate::search_case_sensitive(conents, query, false),
            Vec::<String>::new()
        );

        let query = "";
        let conents = "";
        assert_eq!(
            crate::search_case_sensitive(conents, query, false),
            Vec::<String>::new()
        );
    }
}
