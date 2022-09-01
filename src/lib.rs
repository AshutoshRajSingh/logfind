pub mod config;
pub mod searching;
pub mod constants;
pub mod util;

use config::Config;
use searching::search;
use constants::DEFAULT_PATHS;
use util::flatten_file_tree;

use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for path in DEFAULT_PATHS {
        for entry in flatten_file_tree(path) {
            let contents = match fs::read_to_string(&entry) {
                Ok(contents) => contents,
                Err(_) => {
                    continue;
                }
            };
            let results = search(config.query.as_str(), contents.as_str());
            if results.len() > 0 {
                println!("File: {:?}", &entry);
                for result in results {
                    println!("\t{}: {}", result.line_no, result.line);
                }
            }
            
        }
    }
    Ok(())
}
