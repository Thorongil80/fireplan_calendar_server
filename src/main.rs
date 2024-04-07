use crate::fireplan::monitor_calendars;
use log::{error, info, warn, LevelFilter};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
use std::collections::HashSet;
use std::fs;
use std::sync::mpsc;
use std::thread::JoinHandle;

mod fireplan;

#[derive(Clone, Serialize, Deserialize, Eq, Hash, PartialEq, Debug)]
pub struct Kalender {
    name: String,
    kalender_id: i32,
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
    kalender: Vec<Kalender>,
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

    configuration_output = configuration_output.replace(&configuration.fireplan_api_key, "****");

    info!("Configuration: {}", configuration_output);

    let my_configuration = configuration.clone();
    let handle = std::thread::spawn(move || match monitor_calendars(&my_configuration) {
        Ok(_) => {
            info!("monitor done",)
        }
        Err(e) => {
            error!("monitor failed: {}", e)
        }
    });

    let _ = handle.join();
}
