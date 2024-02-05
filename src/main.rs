// Copyright (c) 2023-2024 Yuki Kishimoto
// Distributed under the MIT software license

use std::str::FromStr;

use nostr_sdk::nostr::nips::nip46::{Message, Request};
use nostr_sdk::prelude::*;

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    // Get secret key
    let secret_key: String = cli::io::get_secret_key()?;
    let my_keys = Keys::from_sk_str(&secret_key)?;

    // Get Nostr Connect URI
    let uri = cli::io::get_input("Nostr Connect URI")?;
    let uri: NostrConnectURI = NostrConnectURI::from_str(&uri)?;

    let client = Client::new(&my_keys);
    client.add_relay(uri.relay_url).await?;

    client.connect().await;

    // Send connect ACK
    let msg = Message::request(Request::Connect(my_keys.public_key()));
    let nip46_event =
        EventBuilder::nostr_connect(&my_keys, uri.public_key, msg)?.to_event(&my_keys)?;
    client.send_event(nip46_event).await?;

    // Subscribe to `App` events
    client
        .subscribe(vec![Filter::new()
            .pubkey(my_keys.public_key())
            .kind(Kind::NostrConnect)
            .since(Timestamp::now())])
        .await;

    println!("\n###############################################\n");
    println!("Listening...");
    println!("\n###############################################\n");

    let mut notifications = client.notifications();
    while let Ok(notification) = notifications.recv().await {
        if let RelayPoolNotification::Event { event, .. } = notification {
            if event.kind == Kind::NostrConnect {
                if let Ok(msg) = decrypt(&my_keys.secret_key()?, &event.pubkey, &event.content) {
                    let msg = Message::from_json(msg)?;

                    println!("New message received: {msg:#?}");
                    println!("\n###############################################\n");
                    if msg.is_request() && cli::io::ask("Approve?")? {
                        if let Some(msg) = msg.generate_response(&my_keys)? {
                            let event = EventBuilder::nostr_connect(&my_keys, event.author(), msg)?
                                .to_event(&my_keys)?;
                            let id = client.send_event(event).await?;
                            println!("\nEvent sent: {id}");
                            println!("\n###############################################\n");
                        }
                    }
                } else {
                    eprintln!("Impossible to decrypt NIP46 message");
                }
            }
        }
    }

    Ok(())
}
