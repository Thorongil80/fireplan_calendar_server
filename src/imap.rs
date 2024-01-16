use std::borrow::Cow;
use std::net::TcpStream;
use std::time::Duration;
use log::{error, info};
use native_tls::{TlsConnector, TlsStream};
use anyhow::{anyhow, Result};
use imap::{Client, Error, Session};
use mail_parser::MessageParser;

pub fn monitor_postbox(config: crate::Standort) -> Result<()> {
    crate::fireplan::submit(config.clone());

    let domain = config.imap_server.as_str();
    let tls = TlsConnector::builder().build()?;
    info!("Connecting {}:{}", domain, config.imap_port);
    let client = imap::connect((domain, config.imap_port), domain, &tls)?;
    info!("Authenticating {},********", config.imap_user);
    let mut imap_session = match client.login(config.imap_user.as_str(), config.imap_password) {
        Ok(s) => {s}
        Err((e, _)) => { return Err(anyhow!(e)) }
    };
    info!("Selecting INBOX");
    imap_session.select("INBOX")?;

    loop {

        if imap_session.

        info!("searching for UNSEEN mails");
        let sequence_set = imap_session.search("UNSEEN");

        match sequence_set {
            Ok(seq) => {
                if seq.len() == 0 {
                    info!("No unread messages found");
                }
                for s in seq {
                    let messages = imap_session.fetch(s.to_string(), "RFC822").unwrap();

                    for message in messages.iter() {
                        if let Some(body) = message.body() {

                            let message = MessageParser::default().parse(body).unwrap();

                            match message.body_text(0) {
                                None => {}
                                Some(s) => {
                                    println!("FROM     : {:?}", message.from());
                                    println!("SUBJECT  : {:?}", message.subject());
                                    println!("RECEIVED : {:?}", message.received());
                                    println!("--------------------------------------------------------");
                                    println!("{}", s);
                                    println!("--------------------------------------------------------");
                                }
                            }

                            match imap_session.store(s.to_string(), "+FLAGS (\\Seen)") {
                                Ok(_) =>  info!("marked message SEEN"),
                                Err(e) => error!("could not mark message as SEEN")
                            };
                        } else {
                            println!("Message didn't have a body!");
                        }
                    }
                }
            }
            Err(e) => { info!("error retrieving messages") }
        }

        match imap_session.idle() {
            Ok(idle) => { info!("engaging IDLE");
                match idle.wait_keepalive() {
                    Ok(_) => { info!("New eMail has arrived"); }
                    Err(_) => { error!("IDLE failed, will wait 10 seconds now"); std::thread::sleep(Duration::from_secs(10)); }
                } }
            Err(e) => { error!("could not initiate IDLE, will wait a minute"); std::thread::sleep(Duration::from_secs(60)); }
        };
    }

    Ok(())
}