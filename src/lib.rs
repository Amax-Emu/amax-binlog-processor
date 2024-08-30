mod common;
//use common::{Vehicle, CITIES, PLAYLISTS, ROUTES, VEHICLES};
use scroll::IOread;

use std::io::SeekFrom;
use std::io::{Cursor, Read, Seek};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod test;

#[derive(Debug)]
pub struct GeneralRaceLog {
    pub playlist_id: u32,
    pub city_id: u32,
    pub route_id: u32,
    pub num_racers: u32,
    pub laps: u32,
    pub time_limit: u32,
    pub players_data: Vec<PlayerData>, //sorted by finish pos
}

#[derive(Debug)]
pub struct PlayerData {
    pub username: String,
    pub dw_id: u64,
    pub traveled_distance: f32,
    pub mod1_id: u32,
    pub mod2_id: u32,
    pub mod3_id: u32,
    pub level: u32,
    pub legend: u32,
    pub vehicle_id: u32,
    pub total_fans: u32,
    pub starting_position: u8,
    pub finish_position: u8,
    pub finish_state: u8,

    pub unk1: f32,
    pub unk2: u32,
}

pub fn parse_general_log(log_data: Vec<u8>) -> Result<GeneralRaceLog, scroll::Error> {
    let mut log = Cursor::new(log_data);

    //Playlist id in Xt. CommunityGroupId in symbols. Game mode from players prespective
    let playlist_id = log.ioread::<u32>()?;

    // after game mode there are bytes
    // 00 00 00 00 07 00 00 00 02 00 00 00

    // Purpose unknown. Game mentions RaceMode which are actual in-game game modes (motor mash, race, etc)
    let _ = log.seek(SeekFrom::Start(16));

    let city_id = log.ioread::<u32>()?;

    let route_id = log.ioread::<u32>()?;

    let num_racers = log.ioread::<u32>()?;
    let laps = log.ioread::<u32>()?;

    let time_limit = log.ioread::<u32>()?;

    //let mut players_data: [Option<PlayerData>;20];

    let mut players_data: Vec<PlayerData> = vec![];

    for _ in 0..num_racers {
        let mut username_buf: [u8; 64] = [0; 64];
        let _ = log.read_exact(&mut username_buf);

        let username_vec16: Vec<u16> = username_buf
            .chunks_exact(2)
            .into_iter()
            .map(|a| u16::from_ne_bytes([a[0], a[1]]))
            .collect();

        //I will allow this unwrap
        let username = String::from_utf16(username_vec16.as_slice())
            .unwrap()
            .trim_matches(char::from(0))
            .to_owned();

        let dw_id = log.ioread::<u64>()?;

        let unk1 = log.ioread::<f32>()?;
        let traveled_distance = log.ioread::<f32>()?;

        let mod1_id = log.ioread::<u32>()?;
        let mod2_id = log.ioread::<u32>()?;
        let mod3_id = log.ioread::<u32>()?;

        let level = log.ioread::<u32>()?;
        let legend = log.ioread::<u32>()?;

        let vehicle_id = log.ioread::<u32>()?;

        let _ = log.ioread::<u32>()?;

        let total_fans = log.ioread::<u32>()?;

        let unk2 = log.ioread::<u32>()?;

        let _ = log.seek(SeekFrom::Current(48)); //Zeroes

        let starting_position = log.ioread::<u8>()? + 1;
        let finish_position = log.ioread::<u8>()?;
        let finish_state = log.ioread::<u8>()?;
        let _ = log.ioread::<u8>()?;

        let data = PlayerData {
            username,
            dw_id,
            traveled_distance,
            mod1_id,
            mod2_id,
            mod3_id,
            level,
            legend,
            vehicle_id,
            total_fans,
            starting_position,
            finish_position,
            finish_state,

            unk1,
            unk2,
        };

        players_data.push(data);
    }

    players_data.sort_by_key(|k| k.finish_position);

    let race_log = GeneralRaceLog {
        playlist_id,
        city_id,
        route_id,
        num_racers,
        laps,
        time_limit,
        players_data,
    };

    Ok(race_log)
}
