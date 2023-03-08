# urbit-notifier

A command-line utility to forward %hark messages from a desk to a webhook URL. This runs best as a background process.

At the moment, this depends on a `config.yml` file to exist in the same directory as the binary (sample provided). The values should be fairly self-explanetory.

## Usage

```
./urbit-notifier <SHIP_URL> <SHIP_NAME> <SHIP_CODE> <DESK> <WEBHOOK>
```

Example:

```
./urbit-notifier http://localhost:80 zod lidlut-tabwed-pillex-ridrup talk http://httpbin.org/post
```

To compile:

```
cargo build --release
```

## Demo

Demonstrating watching %hark events from the `talk` desk and publishing to a Zapier webhook, which sends an email.

![email](https://user-images.githubusercontent.com/748181/223562392-379439ed-53e6-42c8-a386-987b201559aa.gif)

## TODO

- [x] Eliminate the `config.yml` step and accept command-line arguments
- [ ] Allow a user-set polling interval; currently hard-coded to 2s
- [ ] Gracefully handle a non-connection to a ship and derive ship name from login page
