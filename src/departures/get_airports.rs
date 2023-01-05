use rocket::get;
use rocket::serde::json::Json;
use crate::data_builder::build::Airport;
use crate::data_builder::build::read_csv;

type Airports = Vec<Airport>;
#[get("/airports")]
pub async fn get_airports_csv() -> Json<Option<Airports>>{
    let handler = tokio::spawn(async move {
        let records:Vec<Airport> = read_csv("src/data/iata-icao.csv").unwrap();
        let filtered_records: Vec<Airport> = records.into_iter().filter(|v| v.icao !="").collect();
        filtered_records
    });

    if let Some(bytes) = handler.await.ok(){
            Json(Some(bytes))
    }
    else{
            Json(None)
        }
}