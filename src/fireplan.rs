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
use std::io::{read_to_string, Write};
use std::path::Path;
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
struct FireplanKalender {
    kalenderID: i32,
    kalenderName: String,
    standort: String,
}

#[derive(Clone, Serialize, Deserialize, Eq, Hash, PartialEq, Debug)]
struct ApiKey {
    utoken: String,
}

fn fetch_calendars(config: &Configuration) -> Result<(Vec<FireplanKalender>, ApiKey)> {
    info!("Fetch calendars");

    let client = Client::new();
    let token_string = match client
        .get("https://data.fireplan.de/api/Register/Verwaltung".to_string())
        .header("API-Key", config.fireplan_api_key.clone())
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

    let kalender = match serde_json::from_str::<Vec<FireplanKalender>>(&kalender_string) {
        Ok(k) => k,
        Err(e) => return Err(anyhow!("Could not parse calendar list: {}", e)),
    };

    Ok((kalender, token))
}

fn get_calendar(
    fireplan_calendars: &Vec<FireplanKalender>,
    standort: &str,
    name: &str,
    praefix: &str,
    token: &ApiKey,
) -> Result<Calendar> {
    let client = Client::new();

    for calendar in fireplan_calendars {
        info!("{:?}", calendar);
        if calendar.kalenderName.eq(name) && calendar.standort.eq(standort) {
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

            //calendar_out.name(calendar.kalenderName().as_str());

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
                    calendar_out.push(event);
                } else {
                    let event = Event::new()
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

                    info!("{:?}", event);
                    calendar_out.push(event);
                }
            }
            //calendar_out.description(
            //    format!(
            //        "{} of Standort {}",
            //        calendar.kalenderName(),
            //        calendar.standort()
            //    )
            //    .as_str(),
            //);
            calendar_out.timezone("Europe/Berlin");
            //calendar_out.done();
            println!("{}", calendar_out);

            return Ok(calendar_out);
        }
    }

    Err(anyhow!("Nothing found"))
}

fn generate_calendars(
    calendars: Vec<FireplanKalender>,
    token: &ApiKey,
    config: &Configuration,
) -> Result<()> {
    let mut gesamtwehrkalender = get_calendar(
        &calendars,
        "Gesamtwehr",
        "> Gesamtwehrkalender",
        config.praefix_gesamtwehr().as_str(),
        token,
    )?;

    gesamtwehrkalender.description("Abteilungsübergreifende Termine");
    gesamtwehrkalender.name("Gesamtwehrkalender");

    fs::remove_dir_all(Path::new(config.zielordner()))?;
    fs::create_dir_all(Path::new(config.zielordner()))?;

    let mut kalenderdatei = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(format!("{}/Gesamtwehr.ics", config.zielordner().as_str()))
        .unwrap();

    kalenderdatei.write_all(gesamtwehrkalender.to_string().as_bytes())?;
    kalenderdatei.flush()?;

    for konfig_kalender in &config.kalender {
        match get_calendar(
            &calendars,
            konfig_kalender.standort(),
            konfig_kalender.name(),
            konfig_kalender.praefix(),
            &token,
        ) {
            Ok(mut c) => {
                let mut kalenderdatei = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(format!(
                        "{}/{}.ics",
                        config.zielordner(),
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
                    "Could not get calendar {}/{}",
                    konfig_kalender.standort(),
                    konfig_kalender.name()
                );
                continue;
            }
        }
    }

    Ok(())
}

pub fn monitor_calendars(config: &Configuration) -> Result<()> {
    loop {
        match fetch_calendars(&config) {
            Ok((v, token)) => {
                let _ = generate_calendars(v, &token, &config);
            }
            Err(e) => error!("Could not fetch calendars: {}", e),
        };

        std::thread::sleep(Duration::from_secs(900));
    }
}
