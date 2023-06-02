use clap::*;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(name="res2br", author="Ben Raz <ben.raz2008@gmail.com>", about="Converts a resolution (like 1080p) to a bitrate (like 3.000) and optionally allows switching betwen MBPS (default) and KBPS", long_about=None)]
struct Arguments {
    #[arg(help = "Accepts a resolution in ###p format")]
    resolution: String,
    #[arg(short = 'k', long = "use-kbps")]
    #[arg(action=ArgAction::SetTrue)]
    #[arg(help = "Switches to displaying bitrate in KBPS format")]
    use_kbps: Option<bool>,
}

fn main() {
    let arguments = Arguments::parse();

    let table: HashMap<String, f32> = HashMap::from([
        ("1080p".to_string(), 3.000),
        ("720p".to_string(), 1.500),
        ("540p".to_string(), 0.989),
        ("360p".to_string(), 0.460),
        ("270p".to_string(), 0.327),
        ("180p".to_string(), 0.193),
    ]);

    let resolution = &arguments.resolution;
    let is_kbps = match &arguments.use_kbps {
        Some(_) => true,
        None => false,
    };

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
