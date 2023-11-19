## A simple wrapper arround the [fern](https://docs.rs/fern) logger

#### Use example:

cargo.toml
```toml
[dependencies]
logger = {git = "https://github.com/Bowarc/Crates.git", package = "logger"}
log = "0.4.20"
``` 
main.rs
```rust
let cfg = logger::LoggerConfig::new()
    .set_level(log::LevelFilter::Trace)
    .add_filter("wgpu_core", log::LevelFilter::Warn)
    .add_filter("wgpu_hal", log::LevelFilter::Error)
    .add_filter("gilrs", log::LevelFilter::Off)
    .add_filter("naga", log::LevelFilter::Warn)
    .add_filter("networking", log::LevelFilter::Debug)
    .add_filter("ggez", log::LevelFilter::Warn);
// Set the seccond parametter to None if you don't want any log file
logger::init(cfg, Some("log_file.log"));
``` 