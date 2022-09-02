use crate::searching::SearchType;
use crate::constants::FLAG_PREFIX;

pub enum OutputType {
    Default,
    Minimal,
    Verbose
}

pub struct Config <'a> {
    pub queries: Vec<&'a str>,
    pub output_type: OutputType,
    pub search_type: SearchType
}

fn parse_flags<'a>(args: &'a Vec<String>) -> (Vec<&'a String>, Vec<&'a String>) {
    let flags = args.iter().filter(|arg| { arg.starts_with(FLAG_PREFIX) }).collect();
    let queries = args.iter().filter(|arg| { !arg.starts_with(FLAG_PREFIX) }).collect();

    (flags, queries)
}

impl<'a> Config<'a> {
    pub fn new(queries: Vec<&'a str>, output_type: OutputType, search_type: SearchType) -> Self {
        Self { 
                queries: queries,
                output_type: output_type,
                search_type: search_type
            }
    }
    pub fn from_defaults() -> Self {
        Self {
            queries: Vec::new(),
            output_type: OutputType::Default,
            search_type: SearchType::All,
        }
    }
    pub fn from_args(args: &'a Vec<String>) -> Result<Self, String> {
        if args.len() < 2 {
            return Err(String::from("Not enough arguments supplied"));
        } else {
            let (flags, queries) = parse_flags(args);
            let mut new_conf = Config::from_defaults();

            for flag in flags.iter() {
                match flag.as_str() {
                    "--any" => {
                        new_conf.search_type = SearchType::Any;
                    }
                    "--minimal" => {
                        new_conf.output_type = OutputType::Minimal;
                    }
                    _ => {
                        let msg = format!("Unknown flag: {}", flag);
                        return Err(msg);
                    }
                }
            }

            for query in queries[1..].iter() {
                new_conf.queries.push(query);
            }
            Ok(new_conf)
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_config_creation_from_args() {
        let sample_command_line_arguments = vec![
            String::from("logfind"),
            String::from("nginx"),
            String::from("amd64"),
            String::from("--any"),
            String::from("--minimal")
        ];

        let sample_conf = Config::from_args(&sample_command_line_arguments).unwrap();

        assert!(sample_conf.queries.contains(&"nginx"));
        assert!(!sample_conf.queries.contains(&"logfind"));

        if let SearchType::All = sample_conf.search_type {
            panic!("Search type was all when it should have been any");
        }

        if let OutputType::Default = sample_conf.output_type {
            panic!("Output type was default when it shoudl have been minimal");
        }
    }
}
