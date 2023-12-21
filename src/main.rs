mod metadata;
mod nostr;
mod relays;

use metadata::Metadata;
use nostr::Nostr;
use nostr_sdk::prelude::*;
use relays::RELAY_URLS;
use std::{convert::Infallible, env};
use warp::Filter;

const BECH32_SK: &str = "nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85";

#[tokio::main]
async fn main() {
    // todo: reuse &client to save connections
    async fn get_event(event_id: String) -> Result<impl warp::Reply, Infallible> {
        let secret_key = SecretKey::from_bech32(BECH32_SK).expect("no key");
        let my_keys = Keys::new(secret_key);

        let client = Client::new(&my_keys);
        let opt = RelayOptions::new().write(false);
        for relay in RELAY_URLS {
            client
                .add_relay_with_opts(relay, None, opt.clone())
                .await
                .expect("");
        }

        client.connect().await;
        println!("conencted!");

        let instance = Nostr::new(&client);
        let event = instance
            .get_event_by_id(EventId::from_hex(event_id).expect(""))
            .await;

        match event {
            Ok(event) => {
                let handle = Metadata::new(&event);
                Ok(warp::reply::json(
                    &serde_json::json!({"status": "ok", "data": handle.to_meta()}),
                ))
            }
            Err(_err) => Ok(warp::reply::json(
                &serde_json::json!({"status": "not found"}),
            )),
        }
    }

    let default_port = 8080;
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(default_port);
    let route = warp::path!("e" / String).and_then(get_event);
    warp::serve(route)
        .run(([0, 0, 0, 0, 0, 0, 0, 0], port))
        .await;
}
