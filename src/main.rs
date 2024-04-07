use std::collections::HashSet;
use log::{error, info, LevelFilter, warn};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
use std::fs;
use std::sync::mpsc;
use std::thread::JoinHandle;
use crate::fireplan::monitor_calendars;

mod fireplan;

#[derive(Clone, Serialize, Deserialize, Eq, Hash, PartialEq, Debug)]
pub struct Standort {
    standort: String,
    imap_server: String,
    imap_port: u16,
    imap_user: String,
    imap_password: String,
}

#[derive(Clone, Serialize, Deserialize, Eq, Hash, PartialEq, Debug)]
pub struct Ric {
    text: String,
    ric: String,
    subric: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Configuration {
    fireplan_api_key: String,
    regex_einsatzstichwort: String,
    regex_strasse: String,
    regex_ort: String,
    regex_hausnummer: String,
    regex_ortsteil: String,
    regex_einsatznrleitstelle: String,
    regex_koordinaten: String,
    regex_zusatzinfo: String,
    regex_objektname: String,
    rics: Vec<Ric>,
    standorte: Vec<Standort>,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ParsedData {
    rics: Vec<Ric>,
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

fn main() {
    let file = if cfg!(windows) {
        format!(
            "{}\\fireplan_calendar_server.conf",
            std::env::var("USERPROFILE").unwrap()
        )
    } else {
        format!(
            "{}/fireplan_calendar_server.conf",
            homedir::get_my_home().unwrap().unwrap().to_string_lossy()
        )
    };
    let content = fs::read_to_string(file).expect("Config file missing!");
    let configuration: Configuration = toml::from_str(content.as_str()).unwrap();

    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    let mut configuration_output = format!("Configuration: {:?}", configuration);

    for standort in configuration.standorte.clone() {
        configuration_output = configuration_output.replace(&standort.imap_password, "****");
        configuration_output = configuration_output.replace(&configuration.fireplan_api_key, "****");
    }

    info!("Configuration: {}", configuration_output);

    let mut threads: Vec<JoinHandle<()>> = vec![];
    let my_standorte = configuration.standorte.clone();

    for standort in my_standorte {
        let my_standort = standort.clone();
        let my_configuration = configuration.clone();
        let handle = std::thread::spawn(move || {
            match monitor_calendars(&my_configuration) {
                Ok(_) => {
                    info!("monitor done: {}", standort.standort)
                }
                Err(e) => {
                    error!("monitor failed: {}, {}", standort.standort, e)
                }
            }
        });
        threads.push(handle);
    }

    for handle in threads {
        let _ = handle.join();
    }
}
