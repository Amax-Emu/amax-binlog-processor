use std::fs;
use std::path::Path;

#[cfg(test)]
mod tests {
    use crate::{parse_data_gather, parse_general_log};

    use super::*;

    #[test]
    fn test1() {
        let path = Path::new("./test/logData_post_game_6_players");
        let data = fs::read(path).expect("Unable to read file");
        println!("{:?}", parse_general_log(data));
    }

    #[test]
    fn test2() {
        let path = Path::new("./test/logData_with_bots");
        let data = fs::read(path).expect("Unable to read file");
        println!("{:?}", parse_general_log(data));
    }

    #[test]
    fn parse_data_gather_captures() {
        for i in 1..=6 {
            let suffix = if i == 1 { "".to_owned() } else { i.to_string() };
            let path = format!("./test/data_gather/logData_cursed_haruna{suffix}");
            let data = fs::read(path).expect("Unable to read file");
            println!("{:?}", parse_data_gather(data).unwrap());
            
        }
    }
}
