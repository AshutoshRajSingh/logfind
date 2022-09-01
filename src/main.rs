use std::env;
use logfind::run;
use logfind::config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = match Config::from_args(&args){
        Ok(conf) => conf,
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    };
    
    match run(config) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error occurred: {:?}", e);
        }
    }
}
