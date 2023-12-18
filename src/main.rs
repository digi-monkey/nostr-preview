mod nostr;
use std::convert::Infallible;

use nostr::Nostr;
use nostr_sdk::prelude::*;
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
        client
            .add_relay_with_opts("wss://relay.damus.io", None, opt.clone())
            .await
            .expect("");
        client
            .add_relay_with_opts("wss://nostr.wine", None, opt.clone())
            .await
            .expect("");
        client
            .add_relay_with_opts("wss://relay.nostr.info", None, opt.clone())
            .await
            .expect("");
        client
            .add_relay_with_opts("wss://relay.nostr.band", None, opt)
            .await
            .expect("");

        client.connect().await;
        println!("conencted!");

        let instance = Nostr::new(&client);
        let event = instance
            .get_event_by_id(EventId::from_hex(event_id).expect(""))
            .await;

        match event {
            Ok(event) => Ok(warp::reply::json(
                &serde_json::json!({"status": "ok", "data": event}),
            )),
            Err(_err) => Ok(warp::reply::json(
                &serde_json::json!({"status": "not found"}),
            )),
        }
    }

    let route = warp::path!("e" / String).and_then(get_event);
    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
