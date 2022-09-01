use std::fs;
use std::collections::VecDeque;
use std::path::PathBuf;

pub fn flatten_file_tree(filepath: &str) -> Vec<PathBuf> {
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
fn test_directory_flattening() {
    use fs::File;
    const BASE_DIR: &str = "./target/debug/test_dir/";
    let test_dirs = vec![
        "./target/debug/test_dir",
        "./target/debug/test_dir/branch1",
        "./target/debug/test_dir/branch2",
        "./target/debug/test_dir/branch3",
        "./target/debug/test_dir/branch1/subbranch1",
        "./target/debug/test_dir/branch1/subbranch2",
        "./target/debug/test_dir/branch2/subbranch1"
    ];
    let mut test_files = vec![
        "./target/debug/test_dir/root1.log",
        "./target/debug/test_dir/root2.log",
        "./target/debug/test_dir/branch1/lvl1.log",
        "./target/debug/test_dir/branch1/subbranch1/lvl2.log",
        "./target/debug/test_dir/branch2/subbranch1/lvl2.log",
        "./target/debug/test_dir/branch2/lvl1.log",
        "./target/debug/test_dir/branch3/lvl1.log"
    ];

    for dir in test_dirs {
        if let Ok(_) = fs::create_dir_all(dir) {};
    }

    for file in test_files.iter() {
        if let Ok(_) = File::create(file) {};
    }

    let mut flattened = flatten_file_tree(BASE_DIR);

    assert_eq!(test_files.sort(), flattened.sort());

    if let Ok(_) = fs::remove_dir_all(BASE_DIR) {};
}