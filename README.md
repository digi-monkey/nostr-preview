## Nostr Preview

http server for Nostr ogp/event data fetching. It ask data from relays and return it to client in simple HTTP.

the main point is that NostrPreview doesn't store the full event json but only the mapping of event_id => preview metadata so anyone can run it on shitty machines.

todo:
- [x] get event by id
- [ ] reuse relay connections
- [ ] simple memory cache(LRU)
- [ ] get replaceable event by addr
- [ ] choose smart relays to fetch data instead of hardcode
