use glob::glob;
use std::fs::File;
use std::io::{BufReader, Read};
use std::process::{Command, Output};

fn count(output: Output) -> usize {
    let raw_list = String::from_utf8_lossy(&output.stdout);
    let list: Vec<&str> = raw_list.split('\n').into_iter().collect();
    list.iter().count()
}

pub fn packages(manager: &str) -> String {
    match manager {
        "portage" => {
            let file = File::open("/var/lib/portage/world").unwrap();
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents).unwrap();
            let file_vector: Vec<&str> = contents.split('\n').collect();

            let mut list: Vec<String> = Vec::new();
            for entry in glob("/var/db/pkg/*/*/").expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => list.push(path.display().to_string()),
                    Err(e) => println!("{:?}", e),
                }
            }

            let explicit_pre = file_vector.iter().count();
            let explicit = explicit_pre - 1;
            let total = list.iter().count();
            format!("{} (explicit), {} (total)", explicit, total)
        }
        "pacman" => {
            let output = Command::new("pacman")
                .args(&["-Q", "-q"])
                .output()
                .expect("Could not run pacman.");
            let total = count(output);
            format!("{}", total)
        }
        "pip" => {
            let output = Command::new("pip")
                .arg("list")
                .output()
                .expect("Could not run pip.");
            let total = count(output) - 3;
            format!("{}", total)
        }
        _ => format!("N/A ({} is not supported)", manager),
    }
}
