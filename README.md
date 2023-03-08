# urbit-notifier

A command-line utility to forward %hark messages from a desk to a webhook URL. This runs best as a background process.

At the moment, this depends on a `config.yml` file to exist in the same directory as the binary (sample provided). The values should be fairly self-explanetory.

## Usage

```
./urbit-notifier <SHIP_URL> <SHIP_NAME> <SHIP_CODE> <DESK> <WEBHOOK> [INTERVAL]
```

Example:

```
./urbit-notifier http://localhost:80 zod lidlut-tabwed-pillex-ridrup talk http://httpbin.org/post 30
```

This will result in a `POST` request to `http://httpbin.org/post` every 30 seconds with every message from the `talk` desk received within that interval.

Sample payload:

```json
[
  {
    "message": "~dev: .",
    "url": "http://localhost:80/apps/talk/dm/~dev",
    "msg_desk": "talk"
  },
  {
    "message": "~dev: 12345",
    "url": "http://localhost:80/apps/talk/dm/~dev",
    "msg_desk": "talk"
  },
  {
    "message": "~dev: test",
    "url": "http://localhost:80/apps/talk/dm/~dev",
    "msg_desk": "talk"
  }
]
```

To compile:

```
cargo build --release
```

## Demo

> Note: somewhat outdated

Demonstrating watching %hark events from the `talk` desk and publishing to a Zapier webhook, which sends an email.

![email](https://user-images.githubusercontent.com/748181/223562392-379439ed-53e6-42c8-a386-987b201559aa.gif)

## TODO

- [x] Eliminate the `config.yml` step and accept command-line arguments
- [x] Allow a user-set polling interval; currently hard-coded to 2s
- [ ] Gracefully handle a non-connection to a ship and derive ship name from login page
