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

pub fn submit(config: crate::Standort, data: ParsedData) {
    info!("[{}] - Fireplan submit triggered", config.standort);

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

        let client = Client::new();
        match client
            .put(format!(
                "https://data.fireplan.de/api/Register/{}",
                config.standort
            ))
            .header("API-Key", config.fireplan_api_key.clone())
            .header("accept", "*/*")
            .body("the exact body that is sent")
            .send()
        {
            Ok(r) => { println!("{:?}", r); }
            Err(e) => {
                error!("[{}] - Could not get API Key: {}", config.standort, e);
            }
        };
    }
}
