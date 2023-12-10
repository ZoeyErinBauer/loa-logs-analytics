use std::path::Path;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, TimeZone};
use hashbrown::HashMap;
use crate::analytics::models::{UptimeAnalytics, UptimeGraphData};
use crate::get_db_connection;
use crate::parser::models::{Encounter, EncounterDamageStats, EncounterMisc, Skill, StatusEffect};
use crate::parser::models::HitOption::NONE;

fn calculate_synergy_uptime(character_name: String, start_date: NaiveDate, end_date: NaiveDate, resource_path: &Path) -> UptimeAnalytics
{
    let conn = get_db_connection(resource_path).expect("could not get db connection");
    //potentially optimize the join in reverse to use encounters instead of characters
    // first have to check if last_update stores the information on placed date.
    let query = format!("Select
        ent.skill_stats,
        ent.name,
        ent.skills,
        ent.encounter_id,
        enc.fight_start
    FROM entity ent
    LEFT JOIN encounter enc on enc.id = ent.encounter_id
    where ent.name LIKE '%{}%'", character_name);

    let mut encounter_stmt = conn
        .prepare_cached(
            "
    SELECT last_combat_packet,
       fight_start,
       local_player,
       current_boss,
       duration,
       total_damage_dealt,
       top_damage_dealt,
       total_damage_taken,
       top_damage_taken,
       dps,
       buffs,
       debuffs,
       misc,
       difficulty,
       favorite,
       cleared,
       boss_only_damage
    FROM encounter
    WHERE id = ?
    ;",
        )
        .unwrap();
    let mut entity_stmt = conn.prepare_cached(&query).unwrap();
    let mut player_rows = entity_stmt.query([]).unwrap();
    while let (row) = player_rows.next()
    {
        let unwrap_row = row.unwrap().unwrap();
        //party buff logic goes here
        let enc_id = unwrap_row.get(4).unwrap_or_else(|_| "".to_string());
        let encounter_iter = encounter_stmt.query_map([enc_id], |row| {
            let buff_str = row.get(10).unwrap_or_else(|_| "".to_string());
            let buffs = serde_json::from_str::<HashMap<i32, StatusEffect>>(buff_str.as_str())
                .unwrap_or_else(|_| HashMap::new());

            let debuff_str = row.get(11).unwrap_or_else(|_| "".to_string());
            let debuffs = serde_json::from_str::<HashMap<i32, StatusEffect>>(debuff_str.as_str())
                .unwrap_or_else(|_| HashMap::new());

            let misc_str = row.get(12).unwrap_or_else(|_| "".to_string());
            let misc = serde_json::from_str::<EncounterMisc>(misc_str.as_str())
                .map(Some)
                .unwrap_or_else(|_| None);
            Ok(Encounter {
                last_combat_packet: row.get(0)?,
                fight_start: row.get(1)?,
                local_player: row.get(2)?,
                current_boss_name: row.get(3)?,
                duration: row.get(4)?,
                encounter_damage_stats: EncounterDamageStats {
                    total_damage_dealt: row.get(5)?,
                    top_damage_dealt: row.get(6)?,
                    total_damage_taken: row.get(7)?,
                    top_damage_taken: row.get(8)?,
                    dps: row.get(9)?,
                    buffs,
                    debuffs,
                    misc,
                    ..Default::default()
                },
                difficulty: row.get(13)?,
                favorite: row.get(14)?,
                cleared: row.get(15)?,
                boss_only_damage: row.get(16)?,
                ..Default::default()
            })
        }).expect("could not query encounter");
        let mut encounters:Vec<Encounter> = Vec::new();
        let mut data_points = Vec::new();
        for encounter in encounter_iter {
            let unwrap_encounter = encounter.unwrap_or_default();
            let dmg_stats = unwrap_encounter.encounter_damage_stats;
            let encounter_date = NaiveDateTime::from_timestamp_millis(unwrap_encounter.fight_start).unwrap();
            //TODO: Extract date and convert buff uptime to graph objects
            data_points.push(UptimeGraphData {
                data_points: Default::default(),
                data_datetime: encounter_date,
            })
        }




        let skill_str = unwrap_row.get(3).unwrap_or_else(|_| "".to_string());
        let skills = serde_json::from_str::<HashMap<i32, Skill>>(skill_str.as_str())
            .unwrap_or_else(|_| HashMap::new());
        for (key, value) in skills.iter()
        {}
        //ITERATE OVER ENTITY ROWS SORT FILTER ENCOUNTER DATA OUT FOR EACH DATE
    }
    let mut data_points = HashMap::new();
    let mut graph_data = UptimeGraphData {
        data_points,
        data_datetime: Default::default(),
    };
    UptimeAnalytics {
        character_name,
        graph_data,
    }
}