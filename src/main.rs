use std::fs::File;
use log::{info, error, warn, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, Config, debug, TerminalMode, TermLogger, WriteLogger};

mod imap;
mod fireplan;

pub struct Configuration {
    imap_server: String,
    imap_port: u16,
    imap_password: String,
    fireplan_api_key: String
}



fn main() {

    let config : Vec<Configuration> = vec![];

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Trace, Config::default(), File::create("fireplan_alarm_imap.log").unwrap()),
        ]
    ).unwrap();

    error!("Bright red error");
    info!("This only appears in the log file");
    debug!("This level is currently not enabled for any logger");

}
