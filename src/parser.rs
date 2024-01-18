use crate::{Configuration, ParsedData};
use anyhow::{anyhow, Result};

fn parse(body: String, configuration: Configuration) -> Result<Vec<ParsedData>> {
    for lines in body.lines() {

        // find text

        // detect rics by text
    }

    Err(anyhow!("not implemented"))
}
