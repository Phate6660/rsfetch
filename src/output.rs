use clap::ArgMatches;
use nixinfo::{
    cpu, device, distro, env, environment, gpu, hostname, kernel,
    memory_total, memory_used, music, packages, temp, terminal, uptime,
};

fn the_temp(matches: &ArgMatches) -> String {
    let unit = matches.value_of("temperature").unwrap();
    let raw_temp_vec = temp().unwrap();
    // TODO: Is there a better way to handle this without cloning?
    let raw_temp = raw_temp_vec[0].clone();
    if unit == "C" {
        raw_temp.1 + "*C"
    } else if unit == "F" {
        let pre = raw_temp.1.parse::<f64>().unwrap() * 9.0 / 5.0 + 32.0;
        pre.to_string() + "*F"
    } else {
        format!("N/A ({} is not a supported unit)", unit)
    }
}

fn the_memory() -> String {
    let used_memory = memory_used().unwrap();
    let total_memory = memory_total().unwrap();
    format!("{used_memory} (used)/{total_memory} (total)")
}

#[cfg(feature = "pretty_output")]
pub fn main(matches: ArgMatches) {
    let corner: char = if matches.is_present("corner") {
        matches.value_of("corner").unwrap().parse::<char>().unwrap()
    } else {
        '+'
    };
    let mut table = prettytable::Table::new();
    let format = prettytable::format::FormatBuilder::new()
        .column_separator('│')
        .borders('│')
        .padding(1, 1)
        .separators(
            &[prettytable::format::LinePosition::Top],
            prettytable::format::LineSeparator::new('─', '─', corner, corner),
        )
        .separators(
            &[prettytable::format::LinePosition::Bottom],
            prettytable::format::LineSeparator::new('─', '─', corner, corner),
        )
        .build();
    table.set_format(format);
    if matches.is_present("cpu") {
        if matches.is_present("temperature") {
            let temp = the_temp(&matches);
            let row = format!(
                "{} [{}]",
                cpu()
                    .unwrap_or_else(|_| "N/A (could not read /proc/cpuinfo)".to_string())
                    .trim(),
                temp
            );
            table.add_row(row!["CPU", &row]);
        } else {
            table.add_row(row![
                "CPU",
                &cpu().unwrap_or_else(|_| "N/A (could not read /proc/cpuinfo)".to_string())
            ]);
        }
    }
    if matches.is_present("device") {
        table.add_row(row!["Device", &device().unwrap_or_else(|_| "N/A (could not read /sys/devices/virtual/dmi/id/product_name nor /sys/firmware/devicetree/base/model)".to_string())]);
    }
    if matches.is_present("distro") {
        table.add_row(row!["Distro", &distro().unwrap_or_else(|_| "N/A (could not read /bedrock/etc/os-release, /etc/os-release, nor /usr/lib/os-release)".to_string())]);
    }
    if matches.is_present("editor") {
        table.add_row(row!["Editor", &env("EDITOR").unwrap()]);
    }
    if matches.is_present("environment") {
        table.add_row(row!["Environment", &environment().unwrap()]);
    }
    if matches.is_present("gpu") {
        let gpus = &gpu().unwrap();
        for gpu in gpus {
            table.add_row(row!["GPU", gpu]);
        }
    }
    if matches.is_present("hostname") {
        table.add_row(row![
            "Hostname",
            &hostname().unwrap_or_else(|_| "N/A (could not read /etc/hostname)".to_string())
        ]);
    }
    if matches.is_present("kernel") {
        table.add_row(row![
            "Kernel",
            &kernel()
                .unwrap_or_else(|_| "N/A (could not read /proc/sys/kernel/osrelease)".to_string())
        ]);
    }
    if matches.is_present("memory") {
        table.add_row(row![
            "Memory",
            &the_memory()
        ]);
    }
    if matches.is_present("packages") {
        let manager = matches.value_of("packages").unwrap();
        table.add_row(row![
            "Packages",
            &packages(manager).unwrap_or_else(|_| format!("N/A (could not run {})", manager))
        ]);
    }
    if matches.is_present("shell") {
        table.add_row(row!["Shell", &env("SHELL").unwrap()]);
    }
    if matches.is_present("terminal") {
        table.add_row(row![
            "Terminal",
            &terminal().unwrap_or_else(
                |_| "N/A (could not read the appropriate /proc/?/status)".to_string()
            )
        ]);
    }
    if matches.is_present("uptime") {
        table.add_row(row![
            "Uptime",
            &uptime().unwrap_or_else(|_| "N/A (could not read /proc/uptime)".to_string())
        ]);
    }
    if matches.is_present("user") {
        table.add_row(row!["User", &env("USER").unwrap()]);
    }
    if matches.is_present("music") {
        #[cfg(any(feature = "music_mpd", feature = "music_playerctl"))]
        table.add_row(row!["Music", &music().unwrap()]);

        #[cfg(not(any(feature = "music_mpd", feature = "music_playerctl")))]
        table.add_row(row!["Music", &music()]);
    }
    table.printstd();
}

#[cfg(not(feature = "pretty_output"))]
pub fn main(matches: ArgMatches) {
    if matches.is_present("cpu") {
        if matches.is_present("temperature") {
            println!(
                "CPU:          {} [{}]",
                cpu().unwrap_or_else(|_| "N/A (could not read /proc/cpuinfo)".to_string()),
                the_temp(&matches)
            );
        } else {
            println!(
                "CPU:          {}",
                cpu().unwrap_or_else(|_| "N/A (could not read /proc/cpuinfo)".to_string())
            );
        }
    }
    if matches.is_present("device") {
        println!("Device:       {}", device().unwrap_or_else(|_| 
            "N/A (could not read /sys/devices/virtual/dmi/id/product_name nor /sys/firmware/devicetree/base/model)"
            .to_string()
        ));
    }
    if matches.is_present("distro") {
        println!("Distro:       {}", distro().unwrap_or_else(|_| 
            "N/A (could not read /bedrock/etc/os-release, /etc/os-release, nor /usr/lib/os-release)"
            .to_string()
        ));
    }
    if matches.is_present("editor") {
        println!("Editor:       {}", env("EDITOR").unwrap());
    }
    if matches.is_present("environment") {
        println!("Environment:  {}", environment().unwrap());
    }
    if matches.is_present("gpu") {
        let gpus = gpu().unwrap();
        for gpu in gpus {
            println!("GPU:          {}", gpu);
        }
    }
    if matches.is_present("hostname") {
        println!(
            "Hostname:     {}",
            hostname().unwrap_or_else(|_| "N/A (could not read /etc/hostname)".to_string())
        );
    }
    if matches.is_present("kernel") {
        println!(
            "Kernel:       {}",
            kernel()
                .unwrap_or_else(|_| "N/A (could not read /proc/sys/kernel/osrelease)".to_string())
        );
    }
    if matches.is_present("memory") {
        println!(
            "Memory:       {}",
            the_memory()
        );
    }
    if matches.is_present("packages") {
        let manager = matches.value_of("packages").unwrap();
        println!(
            "Packages:     {}",
            packages(manager).unwrap_or_else(|_| format!("N/A (could not run {})", manager))
        );
    }
    if matches.is_present("shell") {
        println!("Shell:        {}", env("SHELL").unwrap());
    }
    if matches.is_present("terminal") {
        println!(
            "Terminal:     {}",
            terminal().unwrap_or_else(
                |_| "N/A (could not read the appropriate /proc/?/status)".to_string()
            )
        );
    }
    if matches.is_present("uptime") {
        println!(
            "Uptime:       {}",
            uptime().unwrap_or_else(|_| "N/A (could not read /proc/uptime)".to_string())
        );
    }
    if matches.is_present("user") {
        println!("User:         {}", env("USER").unwrap());
    }
    if matches.is_present("music") {
        #[cfg(any(feature = "music_mpd", feature = "music_playerctl"))]
        println!(
            "Music:        {}",
            &music()
                .unwrap_or_else(|_| "N/A".to_string())
        );

        #[cfg(not(any(feature = "music_mpd", feature = "music_playerctl")))]
        println!("Music:        {}", music());
    }
}
