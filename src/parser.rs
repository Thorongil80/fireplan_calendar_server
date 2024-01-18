use crate::{Configuration, ParsedData};
use anyhow::{anyhow, Result};

pub fn parse(body: String, configuration: Configuration) -> Result<ParsedData> {
    let mut result = ParsedData {
        rics: vec![],
        einsatznrlst: "".to_string(),
        strasse: "".to_string(),
        hausnummer: "".to_string(),
        ort: "".to_string(),
        ortsteil: "".to_string(),
        objektname: "".to_string(),
        koordinaten: "".to_string(),
        einsatzstichwort: "".to_string(),
        zusatzinfo: "".to_string(),
    };

    for line in body.lines() {
        // find text

        // detect rics by text

        for ric in configuration.rics.clone() {
            if line.contains(ric.text.as_str()) {
                result.rics.push(ric.clone());
            }
        }
    }

    Ok(result)
}
