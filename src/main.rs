use axum::Json;
use hgvroute::{handle_route, RouteRequest};

#[tokio::main]
async fn main() {
    let payload = RouteRequest {
        from_lat: 48.1486,
        from_lon: 17.1077,
        to_lat: 48.7164,
        to_lon: 21.2611,
    };

    match handle_route(Json(payload)).await {
        Ok(Json(response)) => {
            println!("{:#?}", response);
        }
        Err((status, err)) => {
            eprintln!("Error {}: {}", status, err);
        }
    }
}