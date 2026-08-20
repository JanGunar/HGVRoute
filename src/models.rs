use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TruckOptions {
    pub height: f64,
    pub weight: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CostingOptions {
    pub truck: TruckOptions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValhallaRequest {
    pub locations: Vec<Location>,
    pub costing: String,
    pub costing_options: CostingOptions,
}

#[derive(Deserialize, Debug)]
pub struct Summary {
    pub length: f64,
    pub time: f64,
}

#[derive(Deserialize, Debug)]
pub struct Leg {
    pub shape: String,
    pub maneuvers: Vec<Maneuver>,
}

#[derive(Deserialize, Debug)]
pub struct Trip {
    pub summary: Summary,
    pub legs: Vec<Leg>,
}

#[derive(Deserialize, Debug)]
pub struct ValhallaResponse {
    pub trip: Trip,
}

impl ValhallaResponse {
    pub fn get_shape(&self) -> Option<&str> {
        self.trip.legs.first().map(|leg| leg.shape.as_str())
    }
}

#[derive(Debug)]
pub struct HgvSchedule {
    pub driving_time_seconds: u64,
    pub short_breaks_count: u32,
    pub daily_rests_count: u32,
    pub total_duration_seconds: u64,
    pub departure_time: DateTime<Local>,
    pub arrival_time: DateTime<Local>,
}

#[derive(Deserialize, Debug)]
pub struct RouteRequest {
    pub from_lat: f64,
    pub from_lon: f64,
    pub to_lat: f64,
    pub to_lon: f64,
}

#[derive(Serialize, Debug)]
pub struct RouteResponse {
    pub distance_km: f64,
    pub drive_hours: u64,
    pub drive_minutes: u64,
    pub total_hours: u64,
    pub total_minutes: u64,
    pub short_breaks_count: u32,
    pub daily_rests_count: u32,
    pub departure_time: String,
    pub arrival_time: String,
    pub shape: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct Maneuver {
    pub time: f64,
    pub length: f64,
    pub begin_shape_index: usize,
    pub end_shape_index: usize,
}