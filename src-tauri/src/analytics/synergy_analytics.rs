use std::path::Path;
use chrono::NaiveDate;
use hashbrown::HashMap;
use crate::analytics::models::{UptimeAnalytics, UptimeGraphData};
use crate::get_db_connection;

fn calculate_synergy_uptime(character_name: String, start_date: NaiveDate, end_date: NaiveDate, resource_path: &Path ) -> UptimeAnalytics
{

    let conn = get_db_connection(resource_path).expect("could not get db connection");
    //Use connection to build sql query here to get data out for expected entity
    let mut data_points = HashMap::new();
    let mut graph_data = UptimeGraphData {
        data_points
    };
    UptimeAnalytics {
        character_name,
        graph_data,
    }
}