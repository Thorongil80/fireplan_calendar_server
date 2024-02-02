use crate::ParsedData;
use log::{error, info};
use reqwest::blocking::Client;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Eq, Hash, PartialEq, Debug)]
struct FireplanAlarm {
    ric: String,
    subRIC: String,
    einsatznrlst: String,
    strasse: String,
    hausnummer: String,
    ort: String,
    ortsteil: String,
    objektname: String,
    koordinaten: String,
    einsatzstichwort: String,
    zusatzinfo: String,
}

#[derive(Clone, Serialize, Deserialize, Eq, Hash, PartialEq, Debug)]
struct ApiKey {
    utoken: String,
}

pub fn submit(standort: crate::Standort, data: ParsedData) {
    info!("[{}] - Fireplan submit triggered", standort.standort);

    let client = Client::new();
    let token_string = match client
        .get(format!(
            "https://data.fireplan.de/api/Register/{}",
            standort.standort
        ))
        .header("API-Key", standort.fireplan_api_key.clone())
        .header("accept", "*/*")
        .send()
    {
        Ok(r) => {
            println!("{:?}", r);
            if r.status().is_success() {
                match r.text() {
                    Ok(t) => t,
                    Err(e) => {
                        error!("[{}] - Could not get API Key: {}", standort.standort, e);
                        return;
                    }
                }
            } else {
                error!(
                    "[{}] - Could not get API Key: {:?}",
                    standort.standort,
                    r.status()
                );
                return;
            }
        }
        Err(e) => {
            error!("[{}] - Could not get API Key: {}", standort.standort, e);
            return;
        }
    };

    let token: ApiKey = match serde_json::from_str(&token_string) {
        Ok(apikey) => apikey,
        Err(e) => {
            error!("could not deserialize token key: {}", e);
            return;
        }
    };

    info!("[{}] - acquired API Token: {:?}", standort.standort, token);

    for ric in data.rics {
        let alarm = FireplanAlarm {
            ric: ric.ric,
            subRIC: ric.subric,
            einsatznrlst: data.einsatznrlst.clone(),
            strasse: data.strasse.clone(),
            hausnummer: data.hausnummer.clone(),
            ort: data.ort.clone(),
            ortsteil: data.ortsteil.clone(),
            objektname: data.objektname.clone(),
            koordinaten: data.koordinaten.clone(),
            einsatzstichwort: data.einsatzstichwort.clone(),
            zusatzinfo: data.zusatzinfo.clone(),
        };

        info!("[{}] - submitting Alarm: {:?}", standort.standort, alarm);

        match client
            .post("https://data.fireplan.de/api/Alarmierung")
            .header("API-Token", token.utoken.clone())
            .header("accept", "*/*")
            .json(&alarm)
            .send()
        {
            Ok(r) => {
                println!("{:?}", r);
                if r.status().is_success() {
                    match r.text() {
                        Ok(t) => {
                            info!("[{}] - Posted alarm, server says: {}", standort.standort, t)
                        }
                        Err(e) => {
                            error!("[{}] - Could get result text: {}", standort.standort, e);
                            continue;
                        }
                    }
                } else {
                    error!(
                        "[{}] - Could not post alarm: {:?}",
                        standort.standort,
                        r.status()
                    );
                    match r.text() {
                        Ok(t) => info!("[{}] - server says: {}", standort.standort, t),
                        Err(e) => {
                            error!("[{}] - Could not get result text: {}", standort.standort, e);
                            continue;
                        }
                    }
                    continue;
                }
            }
            Err(e) => {
                error!("[{}] - Could not post alarm: {}", standort.standort, e);
                continue;
            }
        }
    }
}
