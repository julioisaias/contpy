## Contrataciones Públicas

A initial implementation of realtime task for contrataciones written in `Rust`.

## Motivation
There are existing implementations such as using the site's own API, or using the mobile application. However, this approach can help create or automate tasks from anywhere.

## How to use

Run with `cargo run`. From here you can listen to all notifications in the Contrataciones Públicas Site.

## Testing

This code can be tested using `cargo test` to run tests and `cargo bench` to run bench tests.

## Dependencies

Add this to your Cargo.toml:

```toml
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
csv = "1.1"
serde = { version = "1", features = ["derive"] }
```

## Disclaimer

There is no relationship with Contrataciones Públicas.

## Contribution

If you like the project, give it a star, or you can contribute.

## License

MIT
