use std::fs;
use std::collections::VecDeque;
use std::path::PathBuf;

fn flatten_file_tree(filepath: &str) -> Vec<PathBuf> {
    let mut resvec: Vec<PathBuf> = Vec::new();
    let mut queue: VecDeque<PathBuf> = VecDeque::new();

    queue.push_back(PathBuf::from(filepath));

    while !queue.is_empty() {
        if let Some(path) = queue.pop_front() {
            let listdir = match fs::read_dir(path){
                Ok(ld) => ld,
                Err(_) => { continue; }
            };

            for entry in listdir {
                let path = entry.unwrap().path();

                if path.is_dir() {
                    queue.push_back(path);
                } else {
                    resvec.push(path);
                }
            }
        }
    }
    resvec
}

#[test]
fn initial_test() {
    let k = flatten_file_tree("/home/wasabi/rust/playground");
    for entry in k {
        println!("{:?}", entry);
    }
}