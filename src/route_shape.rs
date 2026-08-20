use polyline::decode_polyline;
use crate::Maneuver;

pub fn decode_shape(shape: &str) -> Vec<(f64, f64)> {
    let result = polyline::decode_polyline(shape, 6);
    let line_string = match result {
        Ok(ls) => ls,
        Err(_) => return Vec::new(),
    };

    let mut points = Vec::new();

    for cord in line_string {
        points.push((cord.y, cord.x));
    }

    points
}
pub fn find_stop_index(maneuvers: &[Maneuver], target_time_seconds: f64) -> usize {
    let mut elapsed = 0.0;

    for maneuver in maneuvers {
        if elapsed + maneuver.time > target_time_seconds {
            let remaining = target_time_seconds - elapsed;
            let progress = remaining / maneuver.time;
            let index_range = maneuver.end_shape_index - maneuver.begin_shape_index;
            let offset = (progress * index_range as f64) as usize;

            return maneuver.begin_shape_index + offset;
        }
        elapsed += maneuver.time;
    }

    maneuvers.last().map(|m| m.end_shape_index).unwrap_or(0)
}