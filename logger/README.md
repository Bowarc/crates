## A custom logger based on the [log](https://docs.rs/log) crate

### Documentation

The documentation for this crate can be found [here](https://bowarc.github.io/crates/logger)

#### Use example:

Cargo.toml
```toml
[dependencies]
logger = {git = "https://github.com/Bowarc/Crates.git", package = "logger"}
log = "0.4.20"
```

Simple example
main.rs
```rust
logger::init(
    // Initiate the logger with a basic config
    logger::Config::default()
        // Set up default level, Trace is default
        .level(log::LevelFilter::Trace)
        // Chose your output, Stdout is default
        .output(logger::Output::Stdout)
        // Do you want the output to be colored ? default is false
        .colored(false),
);

// This will print "Hi !", without any colors, to the standard output
log::debug!("Hi !");
``` 

More complex example
main.rs
```rust
// OutputSteam is just a trait bundle of std::io::Write + Send + Sync
let custom_output = Box::new(Writer::new()); // Writer is a simple test struct that implements std::io::Write

logger::init(
    // Pass an array of configs to enable multiple loggers
    [
        // Basic stdout colored logger, with Trace as default level
        logger::Config::default()
            .output(logger::Output::Stdout)
            .colored(true),

        // Seccond logger with a custom output, only for the errors
        logger::Config::default()
            .level(log::LevelFilter::Error)
            .output(custom_output.clone())
            .colored(true),

        // Third logger, that outputs to a file, Info as minimal level
        // We also added some filters
        logger::Config::default()
            .level(log::LevelFilter::Info)
            .filter("crate_name", log::LevelFilter::Warn)
            // Here, we do something cool, we set different log levels for different parts of the same crate
            .filter("crate_name::module_name", log::LevelFilter::Trace)
            // This also allows us to use partial crate name, but i don't think that's very useful
            // .filter("crate_na", log::LevelFilter::Error)
            // If you need to reuse filters, you can put them in an array and use the .filters method
            .filters(&[
                ("another_crate_name", log::LevelFilter::Info),
                ("another_crate_name::module_name", log::LevelFilter::Trace),
            ])
            // Output to a file, if the file doesn't exists, it will be created, else we append to it
            .output(std::path::PathBuf::from("test.log")),
    ],
);

debug!("Hi");
// Let's break down what happends here
// - This will be printed to Stdout in color due to the first logger
// - Ignored by the seccond logger, our custom output stream will receive nothing
// - Also be ignored by the third logger, as the minimal level is Info and we diddn't set any filter for "tests"
//   (the crate name the tests run with)

error!("This is an important error");
// Here;
// - This will be printed to Stdout in color due to the first logger
// - The message will also be printed colored to our custom_output stream
// - This message is gonna be written to the test.log file without any color

sleep(Duration::from_millis(1)); // The logger uses a seccond thread to avoid blocking the main one
// so for tests, we need to wait just a bit

// Here we can see, our custom output has only one line
assert!(custom_output.get(0).unwrap().contains("This is an important error")); // I use contains because the time is also printed, which is variable

// It technically has 2 entries, but the 2nd one is the "\n" of the first line, that's how std::io::Write works
// assert_eq!(custom_output.get(1), Some("\n".to_string()));
// assert!(custom_output.get(2).is_none());
   
```
