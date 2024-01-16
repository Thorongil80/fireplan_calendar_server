use std::fs::File;
use log::{info, error, warn, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, Config, debug, TerminalMode, TermLogger, WriteLogger};
use crate::imap::monitor_postbox;

mod imap;
mod fireplan;

#[derive(Clone)]
pub struct Configuration {
    imap_server: String,
    imap_port: u16,
    imap_password: String,
    fireplan_api_key: String
}



fn main() {

    let configuration : Vec<Configuration> = vec![Configuration {
        imap_server: "dummyserver".to_string(),
        imap_port: 1234,
        imap_password: "dummypassword".to_string(),
        fireplan_api_key: "dummykey".to_string(),
    }];

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Trace, Config::default(), File::create("fireplan_alarm_imap.log").unwrap()),
        ]
    ).unwrap();

    for config in configuration {
        monitor_postbox(config.clone());
    }

}
