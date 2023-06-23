mod extract_link;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::from_utf8_unchecked;
use directories:: BaseDirs;
use regex::Regex;

use extract_link::extract_link::from_string;

fn main() -> std::io::Result<()> {
    let playerlog_path = BaseDirs::new().unwrap().home_dir().join(Path::new("AppData\\LocalLow\\Cognosphere\\Star Rail\\Player.log"));
    let mut player_log = File::open(playerlog_path)?;
    let mut player_log_buffer = String::new();
    player_log.read_to_string(&mut player_log_buffer)?;
    let game_path_regex = Regex::new(r".:/.+(StarRail_Data)").unwrap();
    let game_path = game_path_regex.captures_iter(&player_log_buffer).into_iter().last().unwrap().get(0).unwrap().as_str().to_owned();
    println!("{game_path}");
    
    let mut data = File::open("data_2")?;
    let mut buffer = vec![];
    data.read_to_end(&mut buffer)?;

    let data_string = unsafe { from_utf8_unchecked(&buffer) };

    let link_struct = from_string(data_string);

    println!("{:#?}", link_struct);
    Ok(())
}
