use crate::ParsedData;
use log::{error, info};
use reqwest::blocking::{Client, RequestBuilder};
use serde_derive::{Deserialize, Serialize};
use std::io::Read;

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
        .body("the exact body that is sent")
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
                error!("[{}] - Could not get API Key: {:?}", standort.standort, r.status());
                return;
            }
        }
        Err(e) => {
            error!("[{}] - Could not get API Key: {}", standort.standort, e);
            return;
        }
    };

    info!("[{}] - acquired API Key: {}", standort.standort, token_string);

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

    }
}
