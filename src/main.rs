#[macro_use] extern crate rocket;
use rust_airline_tracking::departures::get_depatures::get_departures_airport;
use rust_airline_tracking::arrivals::get_arrivals::get_arrivals_airport;

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build().mount("/", routes![get_departures_airport, get_arrivals_airport]).launch().await?;
    Ok(())
}