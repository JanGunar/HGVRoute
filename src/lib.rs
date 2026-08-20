mod models;
mod route_shape;

pub use models::*;
use axum::{http::StatusCode, routing::post, Json, Router};
use chrono::{DateTime, Local};

pub fn calculate_hgv_schedule(driving_time_seconds: f64, departure_time: DateTime<Local>) -> HgvSchedule {
    let driving_secs = driving_time_seconds as u64;
    let short_breaks = (driving_secs / 16200) as u32;
    let daily_rests = (driving_secs / 32400) as u32;

    let total_secs = driving_secs + (short_breaks as u64 * 2700) + (daily_rests as u64 * 39600);
    let arrival_time = departure_time + chrono::Duration::seconds(total_secs as i64);

    HgvSchedule {
        driving_time_seconds: driving_secs,
        short_breaks_count: short_breaks,
        daily_rests_count: daily_rests,
        total_duration_seconds: total_secs,
        departure_time,
        arrival_time,
    }
}

pub async fn get_valhalla_response(request: &ValhallaRequest) -> Result<ValhallaResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://127.0.0.1:8002/route")
        .json(request)
        .send()
        .await?
        .json::<ValhallaResponse>()
        .await?;
    Ok(res)
}

pub async fn handle_route(
    Json(payload): Json<RouteRequest>,
) -> Result<Json<RouteResponse>, (StatusCode, String)> {
    let origin = Location {
        lat: payload.from_lat,
        lon: payload.from_lon,
    };
    let destination = Location {
        lat: payload.to_lat,
        lon: payload.to_lon,
    };

    let truck_opts = TruckOptions {
        height: 4.0,
        weight: 40.0,
    };
    let costing_opts = CostingOptions { truck: truck_opts };

    let request = ValhallaRequest {
        locations: vec![origin, destination],
        costing: "truck".to_string(),
        costing_options: costing_opts,
    };

    let response = get_valhalla_response(&request)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let departure_time = Local::now();
    let schedule = calculate_hgv_schedule(response.trip.summary.time, departure_time);

    let route_response = RouteResponse {
        distance_km: response.trip.summary.length,
        drive_hours: schedule.driving_time_seconds / 3600,
        drive_minutes: (schedule.driving_time_seconds % 3600) / 60,
        total_hours: schedule.total_duration_seconds / 3600,
        total_minutes: (schedule.total_duration_seconds % 3600) / 60,
        short_breaks_count: schedule.short_breaks_count,
        daily_rests_count: schedule.daily_rests_count,
        departure_time: schedule.departure_time.format("%Y-%m-%d %H:%M").to_string(),
        arrival_time: schedule.arrival_time.format("%Y-%m-%d %H:%M").to_string(),
        shape: response.get_shape().map(|s| s.to_string()),
    };

    Ok(Json(route_response))
}

pub fn create_app() -> Router {
    Router::new().route("/api/route", post(handle_route))
}