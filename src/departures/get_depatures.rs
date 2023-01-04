use rocket::get;
use crate::{API_DEPARTURE,AirportResponses};
use rocket::serde::json::Json;
use std::fmt::Error;
use reqwest::Client;

#[get("/departure/<airport>/<begin>/<end>")]
pub async fn get_departures_airport(airport:String, begin:usize, end:usize) -> Json<Option<AirportResponses>>{
    let client = Client::new();
    let handler = tokio::spawn(async move {
        let resp = client.get(API_DEPARTURE).query(&[("airport", airport), ("begin", begin.to_string()), ("end", end.to_string())])
        .basic_auth( "yuksel".to_owned(),Some("Goksenin123@".to_owned())).send().await.unwrap();
        if resp.status().is_success(){
            let response : AirportResponses = resp.json::<AirportResponses>().await.unwrap();
            Ok(response)
        }
        else{
            Err(Error)
        }
    });

    if let Some(bytes) = handler.await.ok(){
        if let Some(json) = bytes.ok(){
            Json(Some(json))
        }
        else{
            Json(None)
        }
    }
    else{
        Json(None)
    }
}