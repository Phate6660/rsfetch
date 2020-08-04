use glob::glob;
use std::fs::File;
use std::io::{BufReader, Read};
use std::process::{Command, Output};

fn count(output: Output) -> usize {
    let raw_list = String::from_utf8_lossy(&output.stdout);
    let list: Vec<&str> = raw_list.split('\n').collect();
    list.iter().count() - 1 // -1 to deal with newline at end of output
}

pub fn packages(manager: &str) -> String {
    match manager {
        "apk" => {
            let output = Command::new("apk")
                .arg("info")
                .output()
                .expect("Could not run apk.");
            format!("{}", count(output))
        }
        "apt" => {
            let output = Command::new("apt")
                .args(&["list", "--installed"])
                .output()
                .expect("Could not run apt.");
            format!("{}", count(output))
        }
        "dnf" => {
            let output = Command::new("dnf")
                .args(&["list", "installed"])
                .output()
                .expect("Could not run dnf.");
            format!("{}", count(output))
        }
        "eopkg" => {
            let output = Command::new("eopkg")
                .arg("list-installed")
                .output()
                .expect("Could not run eopkg.");
            format!("{}", count(output))
        }
        "pacman" => {
            let output = Command::new("pacman")
                .args(&["-Q", "-q"])
                .output()
                .expect("Could not run pacman.");
            format!("{}", count(output))
        }
        "pip" => {
            let output = Command::new("pip")
                .arg("list")
                .output()
                .expect("Could not run pip.");
            format!("{}", count(output) - 2) // -2 to deal with 2 header lines in output
        }
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

            format!("{} (explicit), {} (total)", file_vector.iter().count() - 1, list.iter().count())
        }
        "rpm" => {
            let output = Command::new("rpm")
                .args(&["-q", "-a"])
                .output()
                .expect("Could not run rpm.");
            format!("{}", count(output))
        }
        "xbps" => {
            let output = Command::new("xbps-query")
                .arg("list-installed")
                .output()
                .expect("Could not run xbps-query.");
            format!("{}", count(output))
        }
        _ => format!("N/A ({} is not supported, please file a bug to get it added!)", manager),
    }
}
