use crate::Configuration;
use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use derive_getters::Getters;
use icalendar::CalendarDateTime::Utc;
use icalendar::{Calendar, CalendarComponent, Class, Component, Event, EventLike, Property};
use log::{error, info};
use reqwest::blocking::Client;
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;
use std::time::Duration;

#[derive(Clone, Serialize, Deserialize, Eq, Hash, PartialEq, Debug, Getters)]
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

#[derive(Clone, Serialize, Deserialize, Eq, Hash, PartialEq, Debug, Getters)]
struct Kalender {
    kalenderID: i32,
    kalenderName: String,
    standort: String,
}

#[derive(Clone, Serialize, Deserialize, Eq, Hash, PartialEq, Debug)]
struct ApiKey {
    utoken: String,
}

fn fetch_calendars(api_key: String) -> Result<(Vec<Kalender>, ApiKey)> {
    info!("Fetch calendars");

    let client = Client::new();
    let token_string = match client
        .get("https://data.fireplan.de/api/Register/Verwaltung".to_string())
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
                        error!("Could not get API Key: {}", e);
                        return Err(anyhow!("Could not get API Key: {}", e));
                    }
                }
            } else {
                error!("Could not get API Key: {:?}", r.status());
                return Err(anyhow!("Could not get API Key: {}", r.status()));
            }
        }
        Err(e) => {
            error!("Could not get API Key: {}", e);
            return Err(anyhow!("Could not get API Key: {}", e));
        }
    };

    let token: ApiKey = match serde_json::from_str(&token_string) {
        Ok(apikey) => apikey,
        Err(e) => {
            error!("could not deserialize token key: {}", e);
            return Err(anyhow!("could not deserialize token key: {}", e));
        }
    };

    info!("acquired API Token: {:?}", token);

    let kalender_string = match client
        .get("https://data.fireplan.de/api/Kalender".to_string())
        .header("API-Token", token.utoken.clone())
        .header("accept", "application/json")
        .send()
    {
        Ok(r) => {
            println!("{:?}", r);
            if r.status().is_success() {
                match r.text() {
                    Ok(t) => t,
                    Err(e) => {
                        error!("Could not get calendar list: {}", e);
                        return Err(anyhow!("Could not get calendar list: {}", e));
                    }
                }
            } else {
                error!("Could not get calendar list: {:?}", r.status());
                return Err(anyhow!("Could not get calendar list: {}", r.status()));
            }
        }
        Err(e) => {
            error!("Could not get calendar list: {}", e);
            return Err(anyhow!("Could not get calendar list: {}", e));
        }
    };

    let kalender = match serde_json::from_str::<Vec<Kalender>>(&kalender_string) {
        Ok(k) => k,
        Err(e) => return Err(anyhow!("Could not parse calendar list: {}", e)),
    };

    Ok((kalender, token))
}

fn generate_calendars(calendars: Vec<Kalender>, token: &ApiKey) -> Result<()> {
    let client = Client::new();

    for calendar in calendars {
        if calendar.kalenderName.contains("> Berichte")
            || calendar.kalenderName.contains("> Gesamtwehrkalender")
        {
            println!("{:?}", calendar);

            let termine_string = match client
                .get(format!(
                    "https://data.fireplan.de/api/Termine/{}",
                    calendar.kalenderID
                ))
                .header("API-Token", token.utoken.clone())
                .header("accept", "application/json")
                .send()
            {
                Ok(r) => {
                    println!("{:?}", r);
                    if r.status().is_success() {
                        match r.text() {
                            Ok(t) => t,
                            Err(e) => {
                                error!("Could not get calendar list: {}", e);
                                return Err(anyhow!("Could not get calendar list: {}", e));
                            }
                        }
                    } else {
                        error!("Could not get calendar list: {:?}", r.status());
                        return Err(anyhow!("Could not get calendar list: {}", r.status()));
                    }
                }
                Err(e) => {
                    error!("Could not get calendar list: {}", e);
                    return Err(anyhow!("Could not get calendar list: {}", e));
                }
            };

            info!("{:?}", termine_string);
            let termine =
                match serde_json::from_str::<Vec<FireplanTermine>>(termine_string.as_str()) {
                    Ok(t) => t,
                    Err(e) => return Err(anyhow!(e)),
                };

            let mut calendar_out = Calendar::new();

            calendar_out.name(calendar.kalenderName().as_str());

            for termin in termine {
                info!("{:?}", termin);
                if *termin.allDay() {
                    let event = Event::new()
                        .all_day(
                            NaiveDate::from_str(termin.startDate.unwrap_or_default().as_str())
                                .unwrap_or_default(),
                        )
                        .summary(termin.subject.unwrap_or_default().as_str())
                        .description(termin.description.unwrap_or_default().as_str())
                        .class(Class::Public)
                        .done();
                    info!("{:?}", event);
                    calendar_out.push(event);
                } else {
                    let event = Event::new()
                        .summary(termin.subject.unwrap_or_default().as_str())
                        .description(termin.description.unwrap_or_default().as_str())
                        .starts(
                            NaiveDate::from_str(termin.startDate.unwrap_or_default().as_str())
                                .unwrap_or_default(),
                        )
                        .class(Class::Public)
                        .ends(
                            NaiveDate::from_str(termin.endDate.unwrap_or_default().as_str())
                                .unwrap_or_default(),
                        )
                        .done();

                    info!("{:?}", event);
                    calendar_out.push(event);
                }
            }
            calendar_out.timezone("Europe/Berlin");
            calendar_out.done();
            let _ = calendar_out.print();
        }
    }

    Ok(())
}

pub fn monitor_calendars(config: &Configuration) -> Result<()> {
    loop {
        match fetch_calendars(config.fireplan_api_key().to_string()) {
            Ok((v, token)) => {
                //info!("{:?}", s);

                let _ = generate_calendars(v, &token);
            }
            Err(e) => error!("Could not fetch calendars: {}", e),
        };

        std::thread::sleep(Duration::from_secs(60));
    }
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
