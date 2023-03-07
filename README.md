# urbit-notifier

A command-line utility to forward %hark messages from a desk to a webhook URL. This runs best as a background process.

At the moment, this depends on a `config.yml` file to exist in the same directory as the binary (sample provided). The values should be fairly self-explanetory.

To compile:

```
cargo build --release
cp config.yml ./target/release
./target/release/urbit-notifier
```

## TODO

- [ ] Eliminate the `config.yml` step and accept command-line arguments
- [ ] Allow a user-set polling interval; currently hard-coded to 2s
- [ ] Gracefully handle a non-connection to a ship
