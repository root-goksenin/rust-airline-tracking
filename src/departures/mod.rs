use serde::{Serialize,Deserialize};
pub mod get_departures;


pub type DepartureResponses = Vec<DepartureResponse>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DepartureResponse {
    icao24: String,
    first_seen: Option<usize>,
    est_departure_airport: Option<String>,
    last_seen: Option<usize>,
    est_arrival_airport: Option<String>,
    callsign: Option<String>,
    est_departure_airport_horiz_distance: Option<usize>,
    est_departure_airport_vert_distance:  Option<usize>,
    est_arrival_airport_horiz_distance:  Option<usize>,
    est_arrival_airport_vert_distance:  Option<usize>,
    departure_airport_candidates_count:  Option<usize>,
    arrival_airport_candidates_count:  Option<usize>,
}