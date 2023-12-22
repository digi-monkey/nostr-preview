mod db;
mod metadata;
mod nostr;
mod relays;

use db::*;
use metadata::Metadata;
use nostr::Nostr;
use nostr_sdk::prelude::*;
use relays::RELAY_URLS;
use std::{convert::Infallible, env};
use warp::Filter;

use crate::metadata::Meta;

const BECH32_SK: &str = "nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85";

// todo: reuse &client/db to save connections
async fn get_event(event_id: String) -> Result<impl warp::Reply, Infallible> {
    let secret_key = SecretKey::from_bech32(BECH32_SK).expect("no key");
    let my_keys = Keys::new(secret_key);
    let db = open_db();
    let read_result = db.get(event_id.clone());
    match read_result {
        Ok(Some(value)) => {
            let value_str = String::from_utf8_lossy(&value).to_string();
            let v = value_str.clone();
            let json_val: Result<Meta, serde_json::Error> = serde_json::from_str(v.as_str());
            match json_val {
                Ok(res) => {
                    println!("hit cache for RocksDB: {:#?}", value_str);
                    return Ok(warp::reply::json(
                        &serde_json::json!({"status": "ok", "data": res}),
                    ));
                }
                Err(err) => {
                    println!("decode cache err: {:#?}", err);
                    return Ok(warp::reply::json(
                        &serde_json::json!({"status": "decode from cache failed"}),
                    ));
                }
            };
        }
        Ok(None) => println!("Key not found in RocksDB"),
        Err(err) => eprintln!("Error reading from RocksDB: {:?}", err),
    }

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
        .get_event_by_id(EventId::from_hex(event_id.clone()).expect(""))
        .await;

    let key = event_id.clone();
    match event {
        Ok(event) => {
            let handle = Metadata::new(&event);
            let data = handle.to_meta();
            let json_str_data = &serde_json::to_string(&data);
            match json_str_data {
                Ok(res) => {
                    db.put(key.as_bytes(), res)
                        .expect("Failed to write to RocksDB");
                }
                Err(_err) => {}
            }

            Ok(warp::reply::json(
                &serde_json::json!({"status": "ok", "data": data}),
            ))
        }
        Err(_err) => Ok(warp::reply::json(
            &serde_json::json!({"status": "not found"}),
        )),
    }
}

#[tokio::main]
async fn main() {
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
