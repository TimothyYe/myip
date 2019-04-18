# myip
An extremely simple Rust web application to get your public IP address.

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