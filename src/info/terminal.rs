use std::fs::File;

fn ppid(file: File) -> String {
    let ppid = crate::shared_functions::line(file, 6);
    ppid.split(':').collect::<Vec<&str>>()[1].to_string()
}

fn name(ppid: String) -> String {
    let path = format!("/proc/{}/status", ppid.trim());
    let file = File::open(path).unwrap();
    let line = crate::shared_functions::line(file, 0);
    line.split(':').collect::<Vec<&str>>()[1].to_string()
}

fn info(process_name: String, process_id: String) -> String {
    if process_name.ends_with("sh")
        || process_name == "ion"
        || process_name == "screen"
        || process_name == "tmux"
        || process_name.starts_with("tmux")
    {
        let path = format!("/proc/{}/status", process_id);
        let new_ppid = ppid(File::open(path).unwrap()).trim().replace("\n", "");
        let new_name = name(new_ppid.clone()).trim().replace("\n", "");
        if new_name.ends_with("sh")
            || new_name == "ion"
            || new_name == "screen"
            || new_name == "tmux"
            || new_name.starts_with("tmux")
        {
            let path = format!("/proc/{}/status", new_ppid);
            let new_ppid = ppid(File::open(path).unwrap()).trim().replace("\n", "");
            name(new_ppid).trim().replace("\n", "")
        } else {
            new_name.trim().replace("\n", "")
        }
    } else {
        process_name.trim().replace("\n", "")
    }
}

pub fn terminal() -> String {
    let id = std::process::id();
    let path = format!("/proc/{}/status", id);
    if std::fs::metadata(path.clone()).is_ok() {
        let process_id = ppid(File::open(path).unwrap()).trim().replace("\n", "");
        let process_name = name(process_id.clone()).trim().replace("\n", "");
        let info = info(process_name, process_id);
        if info == "systemd" || info == "" {
            "N/A (could not determine the terminal, this could be an issue of using tmux)".to_string()
        } else {
            info
        }
    } else {
        format!("N/A (could not read {})", path)
    }
}
