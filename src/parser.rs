use anyhow::{anyhow, Result};
use crate::{Configuration, ParsedData};

fn parse(body: String, configuration: Configuration) -> Result<Vec<ParsedData>> {

    for lines in body.lines() {

        // find text


        // detect rics by text


    }

    Err(anyhow!("not implemented"))

}