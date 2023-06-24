mod extract_link;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::from_utf8_unchecked;
use directories:: BaseDirs;
use regex::Regex;

use extract_link::extract_link::from_string;

fn main() -> std::io::Result<()> {
    // Get game path from log
    // TODO: Manual game path input
    let playerlog_path = BaseDirs::new().unwrap().home_dir().join(Path::new("AppData\\LocalLow\\Cognosphere\\Star Rail\\Player.log"));
    let mut player_log = match File::open(playerlog_path) {
        Ok(log_file) => log_file,
        Err(_) => {
            println!("Player.log file not found. Make sure file exists at AppData/LocalLow/Cognosphere/Star Rail/Player.log");
            return Ok(());
    },
    };
    let mut player_log_buffer = String::new();
    player_log.read_to_string(&mut player_log_buffer)?;

    // Get API link from in-game web browser log
    let game_path_regex = Regex::new(r".:/.+(StarRail_Data)").unwrap();
    let mut data_path = game_path_regex.captures_iter(&player_log_buffer).into_iter().last().unwrap().get(0).unwrap().as_str().to_owned();
    data_path.push_str("/webCaches/Cache/Cache_Data/data_2");
    println!("{data_path}");
    
    let mut buffer = vec![];
    { // Open file scope
        let mut data = File::open(data_path)?;
        data.read_to_end(&mut buffer)?;
    }

    // unsafe because file is not entirely utf-8
    let data_string = unsafe { from_utf8_unchecked(&buffer) };

    let link_struct = from_string(data_string)?;

    println!("{:#?}", link_struct);
    Ok(())
}
