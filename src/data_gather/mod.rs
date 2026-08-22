use scroll::IOread;
use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct DataGatherLog {
    pub host_id: u64,
    pub host_start_time: u32,
    pub user_id: u64,
    pub records: Vec<DataGatherRecord>,
}

#[derive(Debug)]
pub enum DataGatherRecord {
    VehicleInformation(VehicleInformation),
    RankUp(RankUp),
    LobbyChoice(bool),
    QuitRace(QuitRace),
    ModSelection(ModSelection),
    CheckedSessionBoard(SessionboardStats),
    HostMigrated,
    VoteResults(Option<VoteResults>),
    CustomRaceSettings(CustomRaceSettings),
    ChallengeCompleted(ChallengeCompleted),
    AwardsGiven(Vec<AwardInformation>),
    CareerInformation(CareerInformation),
    PostRace(PostRaceStats),
    OnTrackPickups(Vec<TrackPickupData>),
    RegionSetting(u8),
    Empty { category: u8 },
    Unknown { category: u8, data: Vec<u8> },
}

#[derive(Debug)]
pub struct VehicleInformation {
    pub vehicle_id: u32,
    pub respray_id: u32,
}

#[derive(Debug)]
pub struct RankUp {
    pub level: u8,
    pub total_playtime: u64,
    pub events_played: u32,
}

#[derive(Debug)]
pub struct QuitRace {
    pub route_id: u32,
    pub race_mode: u8,
    pub player_count: u8,
}

#[derive(Debug)]
pub struct ModSelection {
    pub loadout_type: u8,
    pub mod1_id: u8,
    pub mod2_id: u8,
    pub mod3_id: u8,
    pub mod1_activations: u16,
    pub mod2_activations: u16,
    pub mod3_activations: u16,
}

#[derive(Debug)]
pub struct SessionboardStats {
    pub position: u8,
    pub race_points: u16,
    pub races_played: u8,
}

#[derive(Debug)]
pub struct VoteResults {
    pub winning_votes: u8,
    pub winning_route: u32,
    pub losing_route: u32,
    pub losing_votes: u8,
}

#[derive(Debug)]
pub struct CustomRaceSettings {
    pub route_id: u32,
    pub lap_limit: u8,
    pub time_limit: u16,
    pub race_mode: u8,
    pub car_class: u8,
    pub damage_mode: u8,
    pub mods: bool,
    pub ai: bool,
    pub respawn: bool,
    pub upgrades: bool,
    pub handicap: bool,
    pub timeout: bool,
    pub lock_to_host_car: bool,
    pub powerups: bool,
    pub powerup_respawn_mode: u8,
    pub random_powerups: bool,
}

#[derive(Debug)]
pub struct ChallengeCompleted {
    pub level: u8,
    pub total_playtime: u64,
    pub events_played: u32,
    pub vehicle_id: u32,
    pub challenges: Vec<CompletedChallenge>,
}

#[derive(Debug)]
pub struct CompletedChallenge {
    pub challenge_type: u8,
    pub tier: u8,
    pub completed_pack: i32,
}

#[derive(Debug)]
pub struct AwardInformation {
    pub award_id: u8,
    pub value: f32,
}

#[derive(Debug)]
pub struct CareerInformation {
    pub level: u8,
    pub driver_score: u32,
    pub powerup_hit_ratio: f32,
    pub total_playtime: u64,
    pub lifetime_fans: u32,
    pub average_position: f32,
    pub cumulative_position: u32,
    pub events_played: u32,
    pub events_won: u32,
}

#[derive(Debug)]
pub struct PostRaceStats {
    pub route_id: u32,
    pub rank: u8,
    pub game_mode: u8,
    pub vehicles_wrecked: u16,
    pub pickups_used: Vec<u16>,
    pub pickups_hit: Vec<u16>,
    pub players_at_start: i8,
    pub players_at_end: i8,
    pub times_wrecked: u16,
    pub hand_of_god_uses: Vec<u8>,
    pub final_position: u8,
    pub distance_to_finish: f32,
    pub motor_mash_points: u16,
    pub timed_out: bool,
    pub timeout_length: u8,
    pub average_lap_time: f32,
    pub race_fans: u16,
    pub showy_flourish_fans: u16,
    pub finish_bonus: u16,
    pub fan_favourite_fans: u16,
    pub challenge_fans: u16,
    pub car_reward_fans: u16,
    pub total_fans: u16,
    pub longest_jump: f32,
    pub longest_drift: f32,
    pub top_speed: f32,
}

#[derive(Debug)]
pub struct TrackPickupData {
    pub id: u32,
    pub times_picked_up: u8,
    pub pickup_type: u8,
}

pub fn parse_data_gather(log_data: Vec<u8>) -> Result<DataGatherLog, scroll::Error> {
    let mut log = Cursor::new(log_data.as_slice());
    let host_id = log.ioread::<u64>()?;
    let host_start_time = log.ioread::<u32>()?;
    let user_id = log.ioread::<u64>()?;
    let mut offset = 20;
    let mut records = Vec::new();

    while offset < log_data.len() {
        if log_data.get(offset) != Some(&b'[') || log_data.get(offset + 2) != Some(&b'=') {
            return Err(scroll::Error::BadInput {
                size: offset,
                msg: "invalid data gathering record header",
            });
        }

        let category = log_data[offset + 1];
        let payload_start = offset + 3;
        let payload_end = log_data[payload_start..]
            .windows(2)
            .position(|bytes| bytes == b"/]")
            .map(|position| payload_start + position)
            .ok_or(scroll::Error::BadInput {
                size: offset,
                msg: "unterminated data gathering record",
            })?;

        records.push(parse_record(
            category,
            &log_data[payload_start..payload_end],
        )?);
        offset = payload_end + 2;
    }

    Ok(DataGatherLog {
        host_id,
        host_start_time,
        user_id,
        records,
    })
}

fn parse_record(category: u8, data: &[u8]) -> Result<DataGatherRecord, scroll::Error> {
    let mut log = Cursor::new(data);

    //handling for a veeeeeery strange behavior
    if data.is_empty() && category != 16 && category != 18 {
        return Ok(DataGatherRecord::Empty { category });
    }

    let record = match category {
        2 => DataGatherRecord::VehicleInformation(VehicleInformation {
            vehicle_id: log.ioread::<u32>()?,
            respray_id: log.ioread::<u32>()?,
        }),
        8 => DataGatherRecord::RankUp(RankUp {
            level: log.ioread::<u8>()?,
            total_playtime: log.ioread::<u64>()?,
            events_played: log.ioread::<u32>()?,
        }),
        9 => DataGatherRecord::LobbyChoice(log.ioread::<u8>()? != 0),
        12 => DataGatherRecord::QuitRace(QuitRace {
            route_id: log.ioread::<u32>()?,
            race_mode: log.ioread::<u8>()?,
            player_count: log.ioread::<u8>()?,
        }),
        13 => DataGatherRecord::ModSelection(ModSelection {
            loadout_type: log.ioread::<u8>()?,
            mod1_id: log.ioread::<u8>()?,
            mod2_id: log.ioread::<u8>()?,
            mod3_id: log.ioread::<u8>()?,
            mod1_activations: log.ioread::<u16>()?,
            mod2_activations: log.ioread::<u16>()?,
            mod3_activations: log.ioread::<u16>()?,
        }),
        15 => DataGatherRecord::CheckedSessionBoard(SessionboardStats {
            position: log.ioread::<u8>()?,
            race_points: log.ioread::<u16>()?,
            races_played: log.ioread::<u8>()?,
        }),
        16 => DataGatherRecord::HostMigrated,
        18 => DataGatherRecord::VoteResults(parse_vote_results(&mut log, data.is_empty())?),
        19 => DataGatherRecord::CustomRaceSettings(parse_custom_race_settings(&mut log)?),
        20 => DataGatherRecord::ChallengeCompleted(parse_challenge_completed(&mut log)?),
        21 => DataGatherRecord::AwardsGiven(parse_awards(&mut log, data.len())?),
        22 => DataGatherRecord::CareerInformation(parse_career_information(&mut log)?),
        23 => DataGatherRecord::PostRace(parse_post_race(&mut log)?),
        24 => DataGatherRecord::OnTrackPickups(parse_track_pickups(&mut log)?),
        26 => DataGatherRecord::RegionSetting(log.ioread::<u8>()?),
        _ => DataGatherRecord::Unknown {
            category,
            data: data.to_vec(),
        },
    };

    if !matches!(
        record,
        DataGatherRecord::Unknown { .. } | DataGatherRecord::Empty { .. }
    ) && log.position() != data.len() as u64
    {
        return Err(scroll::Error::BadInput {
            size: data.len(),
            msg: "unexpected bytes at end of data gathering record",
        });
    }

    Ok(record)
}

fn parse_vote_results(
    log: &mut Cursor<&[u8]>,
    empty: bool,
) -> Result<Option<VoteResults>, scroll::Error> {
    if empty {
        return Ok(None);
    }

    Ok(Some(VoteResults {
        winning_votes: log.ioread::<u8>()?,
        winning_route: log.ioread::<u32>()?,
        losing_route: log.ioread::<u32>()?,
        losing_votes: log.ioread::<u8>()?,
    }))
}

fn parse_custom_race_settings(
    log: &mut Cursor<&[u8]>,
) -> Result<CustomRaceSettings, scroll::Error> {
    Ok(CustomRaceSettings {
        route_id: log.ioread::<u32>()?,
        lap_limit: log.ioread::<u8>()?,
        time_limit: log.ioread::<u16>()?,
        race_mode: log.ioread::<u8>()?,
        car_class: log.ioread::<u8>()?,
        damage_mode: log.ioread::<u8>()?,
        mods: log.ioread::<u8>()? != 0,
        ai: log.ioread::<u8>()? != 0,
        respawn: log.ioread::<u8>()? != 0,
        upgrades: log.ioread::<u8>()? != 0,
        handicap: log.ioread::<u8>()? != 0,
        timeout: log.ioread::<u8>()? != 0,
        lock_to_host_car: log.ioread::<u8>()? != 0,
        powerups: log.ioread::<u8>()? != 0,
        powerup_respawn_mode: log.ioread::<u8>()?,
        random_powerups: log.ioread::<u8>()? != 0,
    })
}

fn parse_challenge_completed(log: &mut Cursor<&[u8]>) -> Result<ChallengeCompleted, scroll::Error> {
    let level = log.ioread::<u8>()?;
    let total_playtime = log.ioread::<u64>()?;
    let events_played = log.ioread::<u32>()?;
    let vehicle_id = log.ioread::<u32>()?;
    let challenge_count = log.ioread::<u32>()?;
    if challenge_count as usize > (log.get_ref().len() - log.position() as usize) / 6 {
        return Err(scroll::Error::BadInput {
            size: challenge_count as usize,
            msg: "invalid challenge count",
        });
    }
    let mut challenges = Vec::with_capacity(challenge_count as usize);

    for _ in 0..challenge_count {
        challenges.push(CompletedChallenge {
            challenge_type: log.ioread::<u8>()?,
            tier: log.ioread::<u8>()?,
            completed_pack: log.ioread::<i32>()?,
        });
    }

    Ok(ChallengeCompleted {
        level,
        total_playtime,
        events_played,
        vehicle_id,
        challenges,
    })
}

fn parse_awards(
    log: &mut Cursor<&[u8]>,
    payload_len: usize,
) -> Result<Vec<AwardInformation>, scroll::Error> {
    if !payload_len.is_multiple_of(5) {
        return Err(scroll::Error::BadInput {
            size: payload_len,
            msg: "invalid awards record size",
        });
    }

    let mut awards = Vec::with_capacity(payload_len / 5);
    while log.position() < payload_len as u64 {
        awards.push(AwardInformation {
            award_id: log.ioread::<u8>()?,
            value: log.ioread::<f32>()?,
        });
    }
    Ok(awards)
}

fn parse_career_information(log: &mut Cursor<&[u8]>) -> Result<CareerInformation, scroll::Error> {
    Ok(CareerInformation {
        level: log.ioread::<u8>()?,
        driver_score: log.ioread::<u32>()?,
        powerup_hit_ratio: log.ioread::<f32>()?,
        total_playtime: log.ioread::<u64>()?,
        lifetime_fans: log.ioread::<u32>()?,
        average_position: log.ioread::<f32>()?,
        cumulative_position: log.ioread::<u32>()?,
        events_played: log.ioread::<u32>()?,
        events_won: log.ioread::<u32>()?,
    })
}

fn read_u16_list(log: &mut Cursor<&[u8]>, count: usize) -> Result<Vec<u16>, scroll::Error> {
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        values.push(log.ioread::<u16>()?);
    }
    Ok(values)
}

fn read_u8_list(log: &mut Cursor<&[u8]>, count: usize) -> Result<Vec<u8>, scroll::Error> {
    let mut values = vec![0; count];
    log.read_exact(&mut values)?;
    Ok(values)
}

fn parse_post_race(log: &mut Cursor<&[u8]>) -> Result<PostRaceStats, scroll::Error> {
    let route_id = log.ioread::<u32>()?;
    let rank = log.ioread::<u8>()?;
    let game_mode = log.ioread::<u8>()?;
    let vehicles_wrecked = log.ioread::<u16>()?;
    let pickup_type_count = log.ioread::<u8>()? as usize;
    let pickups_used = read_u16_list(log, pickup_type_count)?;
    let pickups_hit = read_u16_list(log, pickup_type_count)?;
    let players_at_start = log.ioread::<i8>()?;
    let players_at_end = log.ioread::<i8>()?;
    let times_wrecked = log.ioread::<u16>()?;
    let hand_of_god_type_count = log.ioread::<u8>()? as usize;
    let hand_of_god_uses = read_u8_list(log, hand_of_god_type_count)?;

    Ok(PostRaceStats {
        route_id,
        rank,
        game_mode,
        vehicles_wrecked,
        pickups_used,
        pickups_hit,
        players_at_start,
        players_at_end,
        times_wrecked,
        hand_of_god_uses,
        final_position: log.ioread::<u8>()?,
        distance_to_finish: log.ioread::<f32>()?,
        motor_mash_points: log.ioread::<u16>()?,
        timed_out: log.ioread::<u8>()? != 0,
        timeout_length: log.ioread::<u8>()?,
        average_lap_time: log.ioread::<f32>()?,
        race_fans: log.ioread::<u16>()?,
        showy_flourish_fans: log.ioread::<u16>()?,
        finish_bonus: log.ioread::<u16>()?,
        fan_favourite_fans: log.ioread::<u16>()?,
        challenge_fans: log.ioread::<u16>()?,
        car_reward_fans: log.ioread::<u16>()?,
        total_fans: log.ioread::<u16>()?,
        longest_jump: log.ioread::<f32>()?,
        longest_drift: log.ioread::<f32>()?,
        top_speed: log.ioread::<f32>()?,
    })
}

fn parse_track_pickups(log: &mut Cursor<&[u8]>) -> Result<Vec<TrackPickupData>, scroll::Error> {
    let pickup_count = log.ioread::<u32>()?;
    if pickup_count as usize > (log.get_ref().len() - log.position() as usize) / 6 {
        return Err(scroll::Error::BadInput {
            size: pickup_count as usize,
            msg: "invalid on-track pickup count",
        });
    }
    let mut pickups = Vec::with_capacity(pickup_count as usize);

    for _ in 0..pickup_count {
        pickups.push(TrackPickupData {
            id: log.ioread::<u32>()?,
            times_picked_up: log.ioread::<u8>()?,
            pickup_type: log.ioread::<u8>()?,
        });
    }

    Ok(pickups)
}
