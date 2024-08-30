use std::fs;
use std::fs::File;
use std::path::Path;

#[cfg(test)]
mod tests {
    use crate::parse_general_log;

    use super::*;

    #[test]
    fn test1() {
        let path = Path::new("./test/logData_post_game_6_players");
        let data = fs::read(path).expect("Unable to read file");
        println!("{:?}",parse_general_log(data));
    }

    #[test]
    fn test2() {
        let path = Path::new("./test/logData_with_bots");
        let data = fs::read(path).expect("Unable to read file");
        println!("{:?}",parse_general_log(data));
    }
}