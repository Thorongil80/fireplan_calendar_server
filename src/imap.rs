use log::info;

pub fn monitor_postbox(config: crate::Configuration) {
    info!("monitor fn");
    crate::fireplan::submit(config.clone());
}