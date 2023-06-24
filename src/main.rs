use clap::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;


#[derive(Debug, Serialize, Deserialize)]
struct Config {
   table: HashMap<String, f64>,
   use_kbps_by_default: bool
}

#[derive(Parser, Debug)]
#[command(name="res2br", author="Ben Raz <ben.raz2008@gmail.com>", about="Converts a resolution (like 1080p) to a bitrate (like 3.000) and optionally allows switching betwen MBPS (default) and KBPS", long_about=None)]
struct Arguments {
    #[arg(help = "Accepts a resolution in ###p format")]
    resolution: Option<String>,
    #[arg(short = 't', long = "toggle-kbps")]
    #[arg(action=ArgAction::SetTrue)]
    #[arg(help = "Switches to and from displaying bitrate in KBPS format")]
    toggle_kbps: Option<bool>,
    #[arg(short, long="config-path")]
    #[arg(help = "A path to find the config file at. If left empty, $RES2BR_CONFIG will be used. It is $HOME/.config/res2br/config.json by default. If that file is not present, the default configuration is used.")]
    config_path: Option<String>
}

fn get_config_path(argument: &Option<String>) -> String {
    if argument.is_some() {
        return argument.clone().unwrap();
    }
    if std::env::var("RES2BR_CONFIG").is_ok() {
        return std::env::var("RES2BR_CONFIG").unwrap();
    }
    std::env::set_var("RES2BR_CONFIG", "$HOME/.config/res2br/config.json");
    "$HOME/.config/res2br/config.json".into()
}

fn get_config_from_path(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
   let file_string: String = std::fs::read_to_string(path)?.parse()?;
   let config: Config = serde_json::from_str(&file_string)?;
   Ok(config)
}

fn main() {
    let arguments = Arguments::parse();

    let default_config: Config = Config {
        table: HashMap::from([
        ("1080p".to_string(), 3.000),
        ("720p".to_string(), 1.500),
        ("540p".to_string(), 0.989),
        ("360p".to_string(), 0.460),
        ("270p".to_string(), 0.327),
        ("180p".to_string(), 0.193),
        ]),
        use_kbps_by_default: false
    };
    
    let config_path = get_config_path(&arguments.config_path);
    let user_config = get_config_from_path(&config_path);

    let config: Config = match user_config {
        Ok(config) => config,
        Err(_) => default_config
    };

    let show_resolution = match &arguments.resolution {
        Some(_) => false,
        None => true,
    };

    if show_resolution {
        println!("Possible resolutions:");
        for (key, _) in &config.table {
            print!("{key} ");
        }
        println!();
        std::process::exit(0);
    }

    let resolution = &arguments.resolution.expect("Oops");
    
    let kbps_switched: bool = match &arguments.toggle_kbps {
        Some(value) => *value,
        None => false,
    };

    if !config.table.contains_key(resolution) {
        println!("\"{resolution}\" Is an invalid resolution. Try Again!");
        std::process::exit(1)
    }

    if !config.use_kbps_by_default && !kbps_switched {
        println!("{} Mbps", config.table[resolution]);
        std::process::exit(0);
    }

    println!("{} Kbps", config.table[resolution] * 1000.0); 
}
