use crate::fireplan::hauptschleife;
use derive_getters::Getters;
use log::{error, info, LevelFilter};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
use std::fs;

mod fireplan;

#[derive(Clone, Serialize, Deserialize, Eq, Hash, PartialEq, Debug, Getters)]
pub struct KonfigKalender {
    name: String,
    standort: String,
    praefix: String,
    ical_name: String,
    ical_beschreibung: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Getters)]
pub struct Configuration {
    fireplan_api_key: String,
    praefix_gesamtwehr: String,
    zielordner: String,
    intervall_sekunden: u16,
    kalender: Vec<KonfigKalender>,
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
            homedir::my_home().unwrap().unwrap().to_string_lossy()
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

    let mut configuration_output = format!("Konfiguration: {:?}", configuration);

    configuration_output = configuration_output.replace(&configuration.fireplan_api_key, "****");

    info!("Konfiguration: {}", configuration_output);

    let my_configuration = configuration.clone();
    let handle = std::thread::spawn(move || match hauptschleife(&my_configuration) {
        Ok(_) => {
            info!("Hauptschleife beendet",)
        }
        Err(e) => {
            error!("Hauptschleife konnte nicht starten: {}", e)
        }
    });

    let _ = handle.join();
}
