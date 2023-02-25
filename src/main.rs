// Copyright (c) 2023 Yuki Kishimoto
// Distributed under the MIT software license

use std::str::FromStr;

use nostr_sdk::nostr::nips::nip46::{Message, Request};
use nostr_sdk::prelude::*;

mod cli;

const BECH32_SK: &str = "nsec1dv3n8qff4eujtfy37hu63duujfqv4me5u54a5ucea6jaeqge5jusljqrrk";

#[tokio::main]
async fn main() -> Result<()> {
    let uri = cli::io::get_input("Nostr Connect URI")?;
    let uri: NostrConnectURI = NostrConnectURI::from_str(&uri)?;

    let secret_key = SecretKey::from_bech32(BECH32_SK)?;
    let my_keys = Keys::new(secret_key);

    let client = Client::new(&my_keys);
    client.add_relay(uri.relay_url, None).await?;

    client.connect().await;

    // Send connect ACK
    let msg = Message::request(Request::Connect(my_keys.public_key()));
    let content = encrypt(&my_keys.secret_key()?, &uri.public_key, msg.as_json())?;
    let nip46_event = EventBuilder::new(
        Kind::NostrConnect,
        content,
        &[Tag::PubKey(uri.public_key, None)],
    )
    .to_event(&my_keys)?;
    client.send_event(nip46_event).await?;

    // Subscribe to `App` events
    client
        .subscribe(vec![Filter::new()
            .pubkey(my_keys.public_key())
            .kind(Kind::NostrConnect)
            .since(Timestamp::now())])
        .await;

    let mut notifications = client.notifications();
    while let Ok(notification) = notifications.recv().await {
        if let RelayPoolNotification::Event(_url, event) = notification {
            if event.kind == Kind::NostrConnect {
                if let Ok(msg) = decrypt(&my_keys.secret_key()?, &event.pubkey, &event.content) {
                    let msg = Message::from_json(msg)?;

                    println!("\n###############################################\n");
                    println!("New message received: {msg:#?}");
                    println!("\n###############################################\n");
                    if let Ok(req) = msg.to_request() {
                        if cli::io::ask("Approve?")? {
                            if let Request::SignEvent(unsigned_event) = req {
                                let signed_event = unsigned_event.sign(&my_keys)?;
                                // Send response
                                let sig = serde_json::json!(signed_event.sig);
                                let res = Message::Response {
                                    id: msg.id(),
                                    result: Some(sig),
                                    error: None,
                                };
                                let content =
                                    encrypt(&my_keys.secret_key()?, &event.pubkey, res.as_json())?;
                                let nip46_event = EventBuilder::new(
                                    Kind::NostrConnect,
                                    content,
                                    &[Tag::PubKey(uri.public_key, None)],
                                )
                                .to_event(&my_keys)?;
                                let id = client.send_event(nip46_event).await?;
                                println!("\nEvent sent: {id}")
                            }
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
