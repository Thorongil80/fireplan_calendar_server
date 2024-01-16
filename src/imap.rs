use log::info;

pub fn monitor_postbox(config: crate::Standort) {
    info!("monitor fn");
    crate::fireplan::submit(config.clone());
}