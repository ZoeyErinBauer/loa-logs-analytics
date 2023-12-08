use std::path::Path;
use chrono::NaiveDate;
use hashbrown::HashMap;
use crate::analytics::models::{UptimeAnalytics, UptimeGraphData};
use crate::get_db_connection;

fn calculate_synergy_uptime(character_name: String, start_date: NaiveDate, end_date: NaiveDate, resource_path: &Path) -> UptimeAnalytics
{
    let conn = get_db_connection(resource_path).expect("could not get db connection");
    //potentially optimize the join in reverse to use encounters instead of characters
    // first have to check if last_update stores the information on placed date.
    let query = format!("Select
    e.skill_stats,
    e.name,
    e.encounter_id,
    e.last_update
    FROM entity e
    where e.name LIKE '%'{}'%'", character_name);

    let mut entity_stmt = conn.prepare_cached(&query).unwrap();
    let player_rows = entity_stmt.query([]);
    while let Some(row) = player_rows.next()?
    {
        //ITERATE OVER ENTITY ROWS SORT FILTER ENCOUNTER DATA OUT FOR EACH DATE
    }
    let mut data_points = HashMap::new();
    let mut graph_data = UptimeGraphData {
        data_points
    };
    UptimeAnalytics {
        character_name,
        graph_data,
    }
}