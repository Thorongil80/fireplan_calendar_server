use crate::Configuration;
use anyhow::{anyhow, Result};
use log::{error, info};
use reqwest::blocking::Client;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Eq, Hash, PartialEq, Debug)]
struct FireplanTermine {
    startDate: Option<String>,
    endDate: Option<String>,
    allDay: bool,
    subject: Option<String>,
    location: Option<String>,
    description: Option<String>,
    jahr: Option<String>,
    monat: Option<String>,
    kalenderID: i32,
}

#[derive(Clone, Serialize, Deserialize, Eq, Hash, PartialEq, Debug)]
struct ApiKey {
    utoken: String,
}

pub fn fetch_calendars(standort: String, api_key: String) -> Result<String> {
    info!("[{}] - Fetch calendars", standort);

    let client = Client::new();
    let token_string = match client
        .get(format!(
            "https://data.fireplan.de/api/Register/{}",
            standort
        ))
        .header("API-Key", api_key.clone())
        .header("accept", "*/*")
        .send()
    {
        Ok(r) => {
            println!("{:?}", r);
            if r.status().is_success() {
                match r.text() {
                    Ok(t) => t,
                    Err(e) => {
                        error!("[{}] - Could not get API Key: {}", standort, e);
                        return Err(anyhow!("[{}] - Could not get API Key: {}", standort, e));
                    }
                }
            } else {
                error!("[{}] - Could not get API Key: {:?}", standort, r.status());
                return Err(anyhow!(
                    "[{}] - Could not get API Key: {}",
                    standort,
                    r.status()
                ));
            }
        }
        Err(e) => {
            error!("[{}] - Could not get API Key: {}", standort, e);
            return Err(anyhow!("[{}] - Could not get API Key: {}", standort, e));
        }
    };

    let token: ApiKey = match serde_json::from_str(&token_string) {
        Ok(apikey) => apikey,
        Err(e) => {
            error!("could not deserialize token key: {}", e);
            return Err(anyhow!(
                "[{}] - could not deserialize token key: {}",
                standort,
                e
            ));
        }
    };

    info!("[{}] - acquired API Token: {:?}", standort, token);

    Ok("".to_string())
}

pub fn monitor_calendars(config: &Configuration) -> Result<()> {
    Ok(())
}

// pub fn submit(standort: String, api_key: String, data: ParsedData) {
//     info!("[{}] - Fireplan submit triggered", standort);
//
//     let client = Client::new();
//     let token_string = match client
//         .get(format!(
//             "https://data.fireplan.de/api/Register/{}",
//             standort
//         ))
//         .header("API-Key", api_key.clone())
//         .header("accept", "*/*")
//         .send()
//     {
//         Ok(r) => {
//             println!("{:?}", r);
//             if r.status().is_success() {
//                 match r.text() {
//                     Ok(t) => t,
//                     Err(e) => {
//                         error!("[{}] - Could not get API Key: {}", standort, e);
//                         return;
//                     }
//                 }
//             } else {
//                 error!("[{}] - Could not get API Key: {:?}", standort, r.status());
//                 return;
//             }
//         }
//         Err(e) => {
//             error!("[{}] - Could not get API Key: {}", standort, e);
//             return;
//         }
//     };
//
//     let token: ApiKey = match serde_json::from_str(&token_string) {
//         Ok(apikey) => apikey,
//         Err(e) => {
//             error!("could not deserialize token key: {}", e);
//             return;
//         }
//     };
//
//     info!("[{}] - acquired API Token: {:?}", standort, token);
//
//     for ric in data.rics {
//         let alarm = String::new();
//
//         info!("[{}] - submitting Alarm: {:?}", standort, alarm);
//
//         match client
//             .post("https://data.fireplan.de/api/Alarmierung")
//             .header("API-Token", token.utoken.clone())
//             .header("accept", "*/*")
//             .json(&alarm)
//             .send()
//         {
//             Ok(r) => {
//                 println!("{:?}", r);
//                 if r.status().is_success() {
//                     match r.text() {
//                         Ok(t) => {
//                             info!("[{}] - Posted alarm, server says: {}", standort, t)
//                         }
//                         Err(e) => {
//                             error!("[{}] - Could get result text: {}", standort, e);
//                             continue;
//                         }
//                     }
//                 } else {
//                     error!("[{}] - Could not post alarm: {:?}", standort, r.status());
//                     match r.text() {
//                         Ok(t) => info!("[{}] - server says: {}", standort, t),
//                         Err(e) => {
//                             error!("[{}] - Could not get result text: {}", standort, e);
//                             continue;
//                         }
//                     }
//                     continue;
//                 }
//             }
//             Err(e) => {
//                 error!("[{}] - Could not post alarm: {}", standort, e);
//                 continue;
//             }
//         }
//     }
// }
