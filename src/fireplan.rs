use log::{info};
use serde_derive::{Deserialize, Serialize};
use crate::ParsedData;

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
    zusatzinfo: String
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

    }

}