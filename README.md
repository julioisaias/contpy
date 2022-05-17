## Contrataciones Públicas

A initial implementation of realtime task for contrataciones written in `Rust`. This gets notifications from the EN CONVOCATORIA stage.

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

## How It Works
- Performs an initial data fetch.
- Run a comparison task between current data and previously obtained data.
- If there is a new tender, it shows the new tenders that have been found.

## Disclaimer

There is no relationship with Contrataciones Públicas.

## Contribution

If you like the project, give it a star, or you can contribute.

## License

Distributed under the MIT License. See LICENSE.txt for more information.
