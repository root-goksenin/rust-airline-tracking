#[warn(non_snake_case)]
pub mod departures;
pub mod arrivals;
pub mod data_builder;
use serde::{Serialize,Deserialize};


pub const API_DEPARTURE: &str = "https://opensky-network.org/api/flights/departure?";
pub const API_ARRIVAL: &str = "https://opensky-network.org/api/flights/arrival?";
type AirportResponses = Vec<AirportResponse>;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AirportResponse {
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