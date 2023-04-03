use clap::*;
use std::collections::HashMap;

fn main() {
    let table: HashMap<String, f32> = HashMap::from([
        ("1080p".to_string(), 3.000),
        ("720p".to_string(), 1.500),
        ("540p".to_string(), 0.989),
        ("360p".to_string(), 0.460),
        ("270p".to_string(), 0.327),
        ("180p".to_string(), 0.193),
    ]);

    let command = Command::new("res2br")
        .version("0.5 BETA")
        .author("Ben R. <ben.raz2008@gmail.com")
        .about("Converts a resolution (like 1080p) into a bitrate (like 3.000) and optionally allows switching between Mbps (default) and Kbps")
        .arg(
            Arg::new("resolution")
            .short('r').long("resolution")
            .help("Accepts a resolution in (##p) format")
            .required(true)
        )
        .arg(
            Arg::new("useKbps")
            .short('k').long("usekbps")
            .help("Stwitches from displaying bitrate in Mbps to Kbps").required(false).action(ArgAction::SetTrue)
        )
        .get_matches();

    let resolution = command.get_one::<String>("resolution").expect("Required");

    let is_kbps = command.get_one::<bool>("useKbps").expect("Not Required");

    if !table.contains_key(resolution) {
        println!("\"{resolution}\" Is an invalid resolution. Try Again!");
        std::process::exit(1)
    }

    if !is_kbps {
        println!("{} Mbps", table[resolution]);
        std::process::exit(0);
    }

    println!("{} Kbps", table[resolution] * 1000.0);
}
