#[macro_use] extern crate rocket;

use rust_airline_tracking::departures::get_departures::get_departures_airport;
#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build().mount("/", routes![get_departures_airport]).launch().await?;
    Ok(())
}