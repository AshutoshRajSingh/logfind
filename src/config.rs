pub struct Config {
    pub query: String
}

impl Config {
    pub fn new(query: &String) -> Self {
        Self { query: query.clone() }
    }
    pub fn from_args(args: &[String]) -> Result<Self, &str> {
        if args.len() < 2 {
            return Err("Not enough arguments supplied");
        } else {
            Ok(Self::new(&args[1]))
        }
    }
}

#[test]
fn test_config_creation() {
    let test_string = String::from("This is sus");
    let test_conf = Config::new(&test_string);
    assert_eq!(test_conf.query, test_string);
}

#[test]
fn test_config_creation_from_valid_args() {
    let valid_args = vec![
        String::from("logfind"),
        String::from("nginx")
    ];
    let test_conf = Config::from_args(&valid_args).unwrap();
    assert_eq!(test_conf.query, valid_args[1]);
}

#[test]
fn test_config_creation_from_invalid_args() {
    let invalid_args = vec![
        String::from("logfind"),
    ];
    let other_test_conf = Config::from_args(&invalid_args);
    if let Ok(_) = other_test_conf {
        panic!("config got constructed when it shouldn't have");
    }
}