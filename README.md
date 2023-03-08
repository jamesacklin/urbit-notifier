# urbit-notifier

A command-line utility to forward %hark messages from a desk to a webhook URL. This runs best as a background process.

At the moment, this depends on a `config.yml` file to exist in the same directory as the binary (sample provided). The values should be fairly self-explanetory.

To compile:

```
cargo build --release
cp config.yml ./target/release
./target/release/urbit-notifier
```

## Demo

Demonstrating watching %hark events from the `talk` desk and publishing to a Zapier webhook, which sends an email. 

![email](https://user-images.githubusercontent.com/748181/223562392-379439ed-53e6-42c8-a386-987b201559aa.gif)

## TODO

- [ ] Eliminate the `config.yml` step and accept command-line arguments
- [ ] Allow a user-set polling interval; currently hard-coded to 2s
- [ ] Gracefully handle a non-connection to a ship
