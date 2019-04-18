# myip
An extremely simple Rust web application to get your public IP address.

Visit [https://myip.biturl.top](https://myip.biturl.top) for example.

## Run

```bash
cargo run
```

## Build

```bash
cargo build --release
```

## Docker

```bash
docker run -d --name=myip --restart=always -p 8000:8000 timothyye/myip:latest
```

## License

[MIT License](https://github.com/TimothyYe/myip/blob/master/LICENSE)
