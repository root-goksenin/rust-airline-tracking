use serde::{Serialize,Deserialize};
use std::error::Error;
use super::connect::DatabaseConnector;


#[derive(Debug, Serialize, Deserialize)]
pub struct Airport{
    pub country_code: String,
    pub region_name	: String,
    pub iata: String,
    pub icao:String,
    pub airport: String,
    pub latitude: String,
    pub longitude: String,
}

pub fn read_csv(file_path: &str) ->Result<Vec<Airport>, Box<dyn Error>>  {
        let mut rdr = csv::Reader::from_path(file_path).unwrap();
        let mut airports: Vec<Airport> = Vec::new();
        for result in rdr.deserialize() {
            let record: Airport = result?;
            airports.push(record);
        }
        Ok(airports)
}
pub async fn build_postgres(file_path: &str){
    let connector = DatabaseConnector::new().await.unwrap();
    let records:Vec<Airport> = read_csv(file_path).unwrap();
    let filtered_records: Vec<&Airport> = records.iter().filter(|v| v.icao !="").collect();
    for record in filtered_records.iter(){
        connector.put(record).await;
    }
}