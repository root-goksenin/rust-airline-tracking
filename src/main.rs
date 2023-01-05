#[macro_use] extern crate rocket;
use rust_airline_tracking::departures::get_depatures::get_departures_airport;
use rust_airline_tracking::arrivals::get_arrivals::get_arrivals_airport;
use rust_airline_tracking::departures::get_airports::get_airports_csv;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build().attach(CORS).mount("/", routes![get_departures_airport, get_arrivals_airport,get_airports_csv]).launch().await?;
    Ok(())
}