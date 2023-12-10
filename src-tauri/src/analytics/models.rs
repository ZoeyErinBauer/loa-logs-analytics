use chrono::{DateTime, NaiveDateTime};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UptimeGraphData {
    pub data_points: HashMap<String, f32>,
    pub data_datetime: NaiveDateTime,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UptimeAnalytics {
    pub graph_data: UptimeGraphData,
    pub character_name: String,
}
