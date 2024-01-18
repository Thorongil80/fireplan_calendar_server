use crate::{fireplan, parser};
use anyhow::{anyhow, Result};
use log::{error, info};
use mail_parser::MessageParser;
use native_tls::TlsConnector;
use std::time::Duration;

pub fn monitor_postbox(
    standort: crate::Standort,
    configuration: crate::Configuration,
) -> Result<()> {
    loop {
        let domain = standort.imap_server.as_str();
        let tls = TlsConnector::builder().build()?;
        info!(
            "[{}] - Connecting {}:{}",
            standort.standort, domain, standort.imap_port
        );
        let client = match imap::connect((domain, standort.imap_port), domain, &tls) {
            Ok(c) => c,
            Err(e) => {
                error!(
                    "[{}] - Could not connect: {}, retry in 30 seconds",
                    standort.standort, e
                );
                std::thread::sleep(Duration::from_secs(30));
                continue;
            }
        };
        info!(
            "[{}] - Authenticating {},********",
            standort.standort, standort.imap_user
        );
        let mut imap_session =
            match client.login(standort.imap_user.as_str(), standort.imap_password.clone()) {
                Ok(s) => s,
                Err((e, _)) => return Err(anyhow!(e)),
            };
        info!("[{}] - Selecting INBOX", standort.standort);
        match imap_session.select("INBOX") {
            Ok(_) => {
                info!("[{}] - selected INBOX", standort.standort);
            }
            Err(e) => {
                error!(
                    "[{}] - Select failed, maybe disconnect: {}",
                    standort.standort, e
                );
                std::thread::sleep(Duration::from_secs(10));
                break;
            }
        };

        loop {
            info!("[{}] - searching for UNSEEN mails", standort.standort);
            let sequence_set = imap_session.search("UNSEEN");

            match sequence_set {
                Ok(seq) => {
                    if seq.is_empty() {
                        info!("[{}] - No unread messages found", standort.standort);
                    }
                    for s in seq {
                        let messages = imap_session.fetch(s.to_string(), "RFC822").unwrap();

                        for message in messages.iter() {
                            if let Some(body) = message.body() {
                                let message = MessageParser::default().parse(body).unwrap();

                                match message.body_text(0) {
                                    None => {}
                                    Some(s) => {
                                        info!(
                                            "[{}] - FROM     : {:?}",
                                            standort.standort,
                                            message.from()
                                        );
                                        info!(
                                            "[{}] - SUBJECT  : {:?}",
                                            standort.standort,
                                            message.subject()
                                        );
                                        info!(
                                            "[{}] - RECEIVED : {:?}",
                                            standort.standort,
                                            message.received()
                                        );
                                        info!("[{}] - --------------------------------------------------------", standort.standort);
                                        info!("[{}] - {}", standort.standort, s);
                                        info!("[{}] - --------------------------------------------------------", standort.standort);
                                        info!("[{}] - PARSING...", standort.standort);
                                        match parser::parse(s.to_string(), configuration.clone()) {
                                            Ok(d) => {
                                                info!(
                                                    "[{}] - parsed message: {:?}",
                                                    standort.standort, d
                                                );
                                                let my_data = d.clone();
                                                let my_standort = standort.clone();
                                                std::thread::spawn(move || {
                                                    fireplan::submit(my_standort, my_data);
                                                });
                                            }
                                            Err(e) => {
                                                error!(
                                                    "[{}] - could not parse: {}",
                                                    standort.standort, e
                                                );
                                            }
                                        };
                                    }
                                }

                                match imap_session.store(s.to_string(), "+FLAGS (\\Seen)") {
                                    Ok(_) => info!("[{}] - marked message SEEN", standort.standort),
                                    Err(e) => error!(
                                        "[{}] - could not mark message as SEEN: {}",
                                        standort.standort, e
                                    ),
                                };
                            } else {
                                println!("[{}] - Message didn't have a body!", standort.standort);
                            }
                        }
                    }
                }
                Err(e) => {
                    error!(
                        "[{}] - error retrieving messages: {}, try reconnect in 10s",
                        standort.standort, e
                    );
                    std::thread::sleep(Duration::from_secs(10));
                    break;
                }
            }

            match imap_session.idle() {
                Ok(idle) => {
                    info!("[{}] - engaging IDLE", standort.standort);
                    match idle.wait_keepalive() {
                        Ok(_) => {
                            info!("[{}] - New eMail has arrived", standort.standort);
                        }
                        Err(_) => {
                            error!("[{}] - IDLE failed, maybe disconnect, try reconnect after 10 seconds", standort.standort);
                            std::thread::sleep(Duration::from_secs(10));
                            break;
                        }
                    }
                }
                Err(e) => {
                    error!(
                        "[{}] - could not initiate IDLE: {}, will wait a minute",
                        standort.standort, e
                    );
                    std::thread::sleep(Duration::from_secs(60));
                }
            };
        }
    }
    Ok(())
}
