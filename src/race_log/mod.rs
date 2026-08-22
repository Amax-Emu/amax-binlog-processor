use crate::common::BOT_NAMES;
use scroll::IOread;

use std::io::SeekFrom;
use std::io::{Cursor, Read, Seek};

#[derive(Debug)]
pub struct GeneralRaceLog {
    pub playlist_id: u64,
    pub version: u32,
    pub race_mode: u32,
    pub city_id: u32,
    pub route_id: u32,
    pub num_racers: u32,
    pub laps: u32,
    pub time_limit: i32,
    pub players_data: Vec<PlayerData>, //sorted by finish pos
}

#[derive(Debug)]
pub struct PlayerData {
    pub username: String,
    pub dw_id: u64,
    pub health_remaining: f32,
    pub traveled_distance: f32,
    pub mod1_id: i32,
    pub mod2_id: i32,
    pub mod3_id: i32,
    pub level: i32,
    pub legend: i32,
    pub vehicle_id: u32,
    pub manufacturer_id: u32,
    pub total_fans: u32,
    pub num_powerups: u32,
    pub starting_position: u8,
    pub finish_position: u8,
    pub finish_state: u8,

    pub is_bot: bool,
}

pub fn parse_general_log(log_data: Vec<u8>) -> Result<GeneralRaceLog, scroll::Error> {
    let mut log = Cursor::new(log_data);

    //Playlist id in Xt. CommunityGroupId in symbols. Game mode from players prespective
    let playlist_id = log.ioread::<u64>()?;

    let version = log.ioread::<u32>()?;
    let race_mode = log.ioread::<u32>()?;

    let city_id = log.ioread::<u32>()?;

    let route_id = log.ioread::<u32>()?;

    let num_racers = log.ioread::<u32>()?;
    let laps = log.ioread::<u32>()?;

    let time_limit = log.ioread::<i32>()?;

    let mut players_data: Vec<PlayerData> = vec![];

    for _ in 0..num_racers {
        let mut username_buf: [u8; 64] = [0; 64];
        log.read_exact(&mut username_buf)?;

        let username_vec16: Vec<u16> = username_buf
            .chunks_exact(2)
            .map(|a| u16::from_le_bytes([a[0], a[1]]))
            .collect();

        let username = String::from_utf16(username_vec16.as_slice())
            .map_err(|_| scroll::Error::BadInput {
                size: username_buf.len(),
                msg: "invalid UTF-16 username",
            })?
            .trim_matches(char::from(0))
            .to_owned();

        let is_bot = BOT_NAMES.contains(&username.to_lowercase().as_str());

        let dw_id = log.ioread::<u64>()?;

        let health_remaining = log.ioread::<f32>()?;
        let traveled_distance = log.ioread::<f32>()?;

        // -1 (0xFFFFFFFF) equal no mod equiped
        let mod1_id = log.ioread::<i32>()?;
        let mod2_id = log.ioread::<i32>()?;
        let mod3_id = log.ioread::<i32>()?;

        let level = log.ioread::<i32>()?;
        let legend = log.ioread::<i32>()?;

        let vehicle_id = log.ioread::<u32>()?;

        let manufacturer_id = log.ioread::<u32>()?;

        let total_fans = log.ioread::<u32>()?;

        let num_powerups = log.ioread::<u32>()?;

        let _ = log.seek(SeekFrom::Current(48)); //Zeroes

        let starting_position = log.ioread::<u8>()? + 1;
        let finish_position = log.ioread::<u8>()?;
        let finish_state = log.ioread::<u8>()?;
        let _ = log.ioread::<u8>()?;

        let data = PlayerData {
            username,
            dw_id,
            health_remaining,
            traveled_distance,
            mod1_id,
            mod2_id,
            mod3_id,
            level,
            legend,
            vehicle_id,
            manufacturer_id,
            total_fans,
            num_powerups,
            starting_position,
            finish_position,
            finish_state,

            is_bot,
        };

        players_data.push(data);
    }

    players_data.sort_by_key(|k| k.finish_position);

    let race_log = GeneralRaceLog {
        playlist_id,
        version,
        race_mode,
        city_id,
        route_id,
        num_racers,
        laps,
        time_limit,
        players_data,
    };

    Ok(race_log)
}
