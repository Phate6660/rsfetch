use glob::glob;

pub fn packages(manager: &str) -> String {
    let mut list: Vec<String> = Vec::new();
    match manager {
        "portage" => {
            for entry in glob("/var/db/pkg/*/*/").expect("Failed to read glob pattern") {
                match entry {
                    Ok(path) => list.push(path.display().to_string()),
                    Err(e) => println!("{:?}", e),
                }
            }

            let amount = list.iter().count();
            amount.to_string()
        }
        _ => format!("N/A ({} is not supported)", manager),
    }
}
