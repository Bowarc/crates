#![allow(dead_code)]

#[macro_use]
extern crate log;

use std::{
    io::{self, Write},
    sync::Arc,
    thread::sleep,
    time::Duration,
};

use std::sync::Mutex;

#[derive(Clone)]
struct Writer {
    data: Arc<Mutex<Vec<u8>>>,
    offsets: Arc<Mutex<Vec<usize>>>,
}

impl Writer {
    fn new() -> Self {
        Writer {
            data: Arc::new(Mutex::new(Vec::new())),
            offsets: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn get(&self, index: usize) -> Option<String> {
        let offsets = self.offsets.lock().unwrap(); // Lock the offsets for safe access
        if index < offsets.len() {
            let start = offsets[index];
            let end = if index + 1 < offsets.len() {
                offsets[index + 1]
            } else {
                self.data.lock().unwrap().len() // Lock the data to get its length
            };

            let data = self.data.lock().unwrap(); // Lock the data for safe access
            let slice = &data[start..end];
            std::str::from_utf8(slice).map(|s| s.to_string()).ok() // Convert to String, returning None if invalid UTF-8
        } else {
            None
        }
    }
}

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut data = self.data.lock().unwrap(); // Lock the data for safe access
        let mut offsets = self.offsets.lock().unwrap(); // Lock the offsets for safe access

        offsets.push(data.len());
        data.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

// #[test]
fn config_output() {
    logger::Config::default().try_output("test.log").unwrap();
    logger::Config::default().try_output("../test.log").unwrap();

    // Don't forget mkdir log
    logger::Config::default()
        .try_output("log/test.log")
        .unwrap();
    logger::Config::default()
        .try_output("./log/test.log")
        .unwrap();

    assert_eq!(
        logger::Config::default().try_output("").map(|_| ()),
        Err(logger::ConfigError::InvalidOutput(
            logger::InvalidOutputError::NotAFile
        ))
    );
    assert_eq!(
        logger::Config::default().try_output("/").map(|_| ()),
        Err(logger::ConfigError::InvalidOutput(
            logger::InvalidOutputError::NotAFile
        ))
    );
    assert_eq!(
        logger::Config::default().try_output("./").map(|_| ()),
        Err(logger::ConfigError::InvalidOutput(
            logger::InvalidOutputError::NotAFile
        ))
    );

    assert_eq!(
        logger::Config::default()
            .try_output("./this_directory_does_not_exists/test.log")
            .map(|_| ()),
        Err(logger::ConfigError::InvalidOutput(
            logger::InvalidOutputError::DirectoryDoesNotExist
        ))
    );
    assert_eq!(
        logger::Config::default()
            .try_output("/test.log")
            .map(|_| ()),
        Err(logger::ConfigError::InvalidOutput(
            logger::InvalidOutputError::ReadOnlyDirectory
        ))
    );
}

// #[test]
fn _test() {
    let output1 = Box::new(Writer::new());
    let output2 = Box::new(Writer::new());
    let output3 = Box::new(Writer::new());

    let _lh = logger::init([
        logger::Config::default()
            .output(logger::Output::CustomStream(output1.clone()))
            .filter("test", log::LevelFilter::Warn),
        logger::Config::default()
            .output(logger::Output::CustomStream(output2.clone()))
            .filter("t", log::LevelFilter::Info),
        logger::Config::default()
            .output(logger::Output::CustomStream(output3.clone()))
            .level(log::LevelFilter::Off),
    ]);

    /*
        Here, we can see that none of the writer has any data
        This means that;
        - logger 1, it's filter Warn on "test" has blocked a debug comming from "text"
        - logger 2, it's filter Info on "t" has blocked a debug comming from "text"
        - logger 2, it's global level of Off has blocked a debug comming from "text"
    */
    debug!("Hi");
    sleep(Duration::from_millis(10));
    assert!(output1.get(0).is_none());
    assert!(output2.get(0).is_none());
    assert!(output3.get(0).is_none());

    /*
        Here, we can see that the first and seccond writer has data
        This means that;
        - logger 1, it's filter Warn on "test" has blocked a debug comming from "text"
        - logger 2, it's filter Info on "t" has blocked a debug comming from "text"
        - logger 2, it's global level of Off has blocked a debug comming from "text"
    */
    error!("Hi");
    sleep(Duration::from_millis(10));
    assert!(output1.get(0).is_some());
    assert!(output2.get(0).is_some());
    assert!(output3.get(0).is_none());
}

#[test]
fn _readme_simple() {
    let _lhandle = logger::init(
        // Initiate the logger with a basic config
        logger::Config::default()
            // Set up default level, Trace is default
            .level(log::LevelFilter::Trace)
            // Chose your output, Stdout is default
            .output(logger::Output::Stdout)
            // Do you want the output to be colored ? default is false
            .colored(false),
    );

    log::debug!("Hi :3");
}

// #[test]
fn _readme_advanced() {
    // OutputSteam is just a trait bundle of std::io::Write + Send + Sync
    let custom_output = Box::new(Writer::new());

    let _lh = logger::init(
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
    assert!(custom_output
        .get(0)
        .unwrap()
        .contains("This is an important error")); // I use contains because the time is also printed, which is variable

    // It technically has 2 entries, but the 2nd one is the "\n" of the first line, that's how std::io::Write works
    // assert_eq!(custom_output.get(1), Some("\n".to_string()));
    // assert!(custom_output.get(2).is_none());
}

// #[test]
fn _timed_file() {
    let _lh= logger::init(
        logger::Config::default().output(logger::Output::new_timed_file(
            "timed.log",
            Duration::from_secs(1),
        )),
    );

    debug!("Hiiii");
    error!("Error1");
    sleep(Duration::from_secs_f32(1.1));

    debug!("Sup");
    error!("error 2");

    sleep(Duration::from_secs_f32(1.9));

    debug!("holaa");
    error!("error 3");

    sleep(Duration::from_millis(1))
}

