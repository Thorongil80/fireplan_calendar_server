#![allow(non_snake_case)]

use crate::Configuration;
use anyhow::{anyhow, Result};
use chrono::{NaiveDate, NaiveDateTime};
use derive_getters::Getters;
use icalendar::{Calendar, Class, Component, Event, EventLike};
use log::{error, info};
use reqwest::blocking::Client;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
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
struct FireplanKalender {
    kalenderID: i32,
    kalenderName: String,
    standort: String,
}

#[derive(Clone, Serialize, Deserialize, Eq, Hash, PartialEq, Debug)]
struct ApiKey {
    utoken: String,
}

fn hole_kalenderliste(konfig: &Configuration) -> Result<(Vec<FireplanKalender>, ApiKey)> {
    info!("Kalenderliste holen");

    let client = Client::new();
    let token_string = match client
        .get("https://data.fireplan.de/api/Register/Verwaltung".to_string())
        .header("API-Key", konfig.fireplan_api_key.clone())
        .header("accept", "*/*")
        .send()
    {
        Ok(r) => {
            println!("{:?}", r);
            if r.status().is_success() {
                match r.text() {
                    Ok(t) => t,
                    Err(e) => {
                        error!("Konnte ApiKey nicht bekommen: {}", e);
                        return Err(anyhow!("Konnte ApiKey nicht bekommen: {}", e));
                    }
                }
            } else {
                error!("Konnte ApiKey nicht bekommen: {:?}", r.status());
                return Err(anyhow!("Konnte ApiKey nicht bekommen: {}", r.status()));
            }
        }
        Err(e) => {
            error!("Konnte ApiKey nicht bekommen: {}", e);
            return Err(anyhow!("Konnte ApiKey nicht bekommen: {}", e));
        }
    };

    let token: ApiKey = match serde_json::from_str(&token_string) {
        Ok(apikey) => apikey,
        Err(e) => {
            error!("Konnte ApiToken nicht deserialisieren: {}", e);
            return Err(anyhow!("Konnte ApiToken nicht deserialisieren: {}", e));
        }
    };

    info!("ApiToken erhalten: {:?}", token);

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
                        error!("Konnte Kalenderliste nicht laden: {}", e);
                        return Err(anyhow!("Konnte Kalenderliste nicht laden: {}", e));
                    }
                }
            } else {
                error!("Konnte Kalenderliste nicht laden: {:?}", r.status());
                return Err(anyhow!("Konnte Kalenderliste nicht laden: {}", r.status()));
            }
        }
        Err(e) => {
            error!("Konnte Kalenderliste nicht laden: {}", e);
            return Err(anyhow!("Konnte Kalenderliste nicht laden: {}", e));
        }
    };

    let kalender = match serde_json::from_str::<Vec<FireplanKalender>>(&kalender_string) {
        Ok(k) => k,
        Err(e) => return Err(anyhow!("Konnte Kalenderliste nicht entschlüsseln: {}", e)),
    };

    Ok((kalender, token))
}

fn hole_kalender(
    kalenderliste: &Vec<FireplanKalender>,
    standort: &str,
    name: &str,
    praefix: &str,
    token: &ApiKey,
) -> Result<Calendar> {
    let klient = Client::new();

    for calendar in kalenderliste {
        info!("{:?}", calendar);
        if calendar.kalenderName.eq(name) && calendar.standort.eq(standort) {
            let termine_string = match klient
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
                                error!("Konnte Kalender nicht laden: {}", e);
                                return Err(anyhow!("Konnte Kalender nicht laden: {}", e));
                            }
                        }
                    } else {
                        error!("Konnte Kalender nicht laden: {:?}", r.status());
                        return Err(anyhow!("Konnte Kalender nicht laden: {}", r.status()));
                    }
                }
                Err(e) => {
                    error!("Konnte Kalender nicht laden: {}", e);
                    return Err(anyhow!("Konnte Kalender nicht laden: {}", e));
                }
            };

            info!("{:?}", termine_string);
            let termine =
                match serde_json::from_str::<Vec<FireplanTermine>>(termine_string.as_str()) {
                    Ok(t) => t,
                    Err(e) => return Err(anyhow!(e)),
                };

            let mut kalender_ausgabe = Calendar::new();

            for termin in termine {
                info!("{:?}", termin);
                if *termin.allDay() {
                    let event = Event::new()
                        .all_day(
                            NaiveDate::parse_from_str(
                                termin.startDate.unwrap_or_default().as_str(),
                                "%m/%d/%Y %_I:%M:%S %P",
                            )
                            .unwrap_or_default(),
                        )
                        .summary(
                            format!("{}: {}", praefix, termin.subject.unwrap_or_default()).as_str(),
                        )
                        .description(termin.description.unwrap_or_default().as_str())
                        .class(Class::Public)
                        .done();
                    info!("{:?}", event);
                    kalender_ausgabe.push(event);
                } else {
                    let ereignis = Event::new()
                        .summary(
                            format!("{}: {}", praefix, termin.subject.unwrap_or_default()).as_str(),
                        )
                        .description(termin.description.unwrap_or_default().as_str())
                        .starts(
                            NaiveDateTime::parse_from_str(
                                termin.startDate.unwrap_or_default().as_str(),
                                "%m/%d/%Y %_I:%M:%S %P",
                            )
                            .unwrap_or_default(),
                        )
                        .class(Class::Public)
                        .ends(
                            NaiveDate::parse_from_str(
                                termin.endDate.unwrap_or_default().as_str(),
                                "%m/%d/%Y %_I:%M:%S %P",
                            )
                            .unwrap_or_default(),
                        )
                        .done();

                    info!("{:?}", ereignis);
                    kalender_ausgabe.push(ereignis);
                }
            }

            kalender_ausgabe.timezone("Europe/Berlin");
            //calendar_out.done();
            println!("{}", kalender_ausgabe);

            return Ok(kalender_ausgabe);
        }
    }

    Err(anyhow!("Nichts gefunden"))
}

fn generiere_kalender(
    kalender: Vec<FireplanKalender>,
    token: &ApiKey,
    konfig: &Configuration,
) -> Result<()> {
    let mut gesamtwehrkalender = hole_kalender(
        &kalender,
        "Gesamtwehr",
        "> Gesamtwehrkalender",
        konfig.praefix_gesamtwehr().as_str(),
        token,
    )?;

    gesamtwehrkalender.description("Abteilungsübergreifende Termine");
    gesamtwehrkalender.name("Gesamtwehrkalender");

    fs::remove_dir_all(Path::new(konfig.zielordner()))?;
    fs::create_dir_all(Path::new(konfig.zielordner()))?;

    let mut kalenderdatei = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(format!("{}/Gesamtwehr.ics", konfig.zielordner().as_str()))
        .unwrap();

    kalenderdatei.write_all(gesamtwehrkalender.to_string().as_bytes())?;
    kalenderdatei.flush()?;

    for konfig_kalender in &konfig.kalender {
        match hole_kalender(
            &kalender,
            konfig_kalender.standort(),
            konfig_kalender.name(),
            konfig_kalender.praefix(),
            token,
        ) {
            Ok(mut c) => {
                let mut kalenderdatei = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(format!(
                        "{}/{}.ics",
                        konfig.zielordner(),
                        konfig_kalender.ical_name()
                    ))
                    .unwrap();

                c.description(konfig_kalender.ical_name());
                c.description(konfig_kalender.ical_beschreibung());

                kalenderdatei.write_all(c.to_string().as_bytes())?;
                kalenderdatei.flush()?;
            }
            Err(e) => {
                error!(
                    "Konnte Kalender {}/{} nicht laden: {}",
                    konfig_kalender.standort(),
                    konfig_kalender.name(),
                    e
                );
                continue;
            }
        }
    }

    Ok(())
}

pub fn hauptschleife(konfig: &Configuration) -> Result<()> {
    loop {
        match hole_kalenderliste(konfig) {
            Ok((v, token)) => {
                let _ = generiere_kalender(v, &token, konfig);
            }
            Err(e) => error!("Konnte Kalenderliste nicht laden: {}", e),
        };

        std::thread::sleep(Duration::from_secs(900));
    }
}
