
use tokio_postgres::{NoTls, Error};
use tokio_postgres::Client;
use serde::Deserialize;
use std::fmt;
use std::fmt::Display;
use super::build::Airport;
pub struct DatabaseConnector{
    client : Client,
}
struct DatabaseConfig;

#[derive(Deserialize, Debug)]
struct Config{
    host:String,
    port:i32,
    user:String,
    password:String,
    database:String,
}
impl Display for Config{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "host={} port={} user={} password={} dbname={}", self.host, self.port, self.user, self.password, self.database)
    }
}
impl DatabaseConfig{
    pub fn config() -> String{
        dotenv::from_filename("src/.env").ok();
        let optional: Option<Config>= match envy::from_env::<Config>() {
            Ok(config) => Some(config),
            Err(error) => {println!("Error connecting {}", error);None}
        };
        optional.unwrap().to_string()
}
}
impl DatabaseConnector{
    pub async fn new() -> Result<DatabaseConnector,Error>{
            // Connect to the database.
            let (client, connection) =
                tokio_postgres::connect(&DatabaseConfig::config(), NoTls).await?;
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("connection error: {}", e);
                }
            });

            let database  = DatabaseConnector{
                client :client};
            Ok(
                database
            )
        }
    
    pub async fn put(&self, record: &Airport) {
        self.client.execute("INSERT INTO airport (country_code,region_name,iata,icao,airport,latitude,longitude)  VALUES ($1, $2, $3,$4, $5, $6,$7)",  &[&record.country_code, &record.region_name, &record.iata, &record.icao, &record.airport, &record.latitude, &record.longitude])
        .await.unwrap();
    }
    pub async fn delete(&self, column: &str, val:&str){
        self.client.execute("delete from airport
        where $1 = $2", &[&column, &val])
        .await.unwrap();
    }
    }