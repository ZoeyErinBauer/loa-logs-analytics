use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Default,Debug,Clone,PartialEq,Serialize,Deserialize)]
pub struct UptimeGraphData{
    pub data_points : HashMap<String,f32>
}