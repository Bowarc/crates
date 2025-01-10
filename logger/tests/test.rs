#[macro_use]
extern crate log;

use std::{
    io::{self, Write},
    sync::Arc,
    thread::sleep,
    time::Duration,
};

use parking_lot::Mutex;

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
        let offsets = self.offsets.lock(); // Lock the offsets for safe access
        if index < offsets.len() {
            let start = offsets[index];
            let end = if index + 1 < offsets.len() {
                offsets[index + 1]
            } else {
                self.data.lock().len() // Lock the data to get its length
            };

            let data = self.data.lock(); // Lock the data for safe access
            let slice = &data[start..end];
            std::str::from_utf8(slice).map(|s| s.to_string()).ok() // Convert to String, returning None if invalid UTF-8
        } else {
            None
        }
    }
}

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut data = self.data.lock(); // Lock the data for safe access
        let mut offsets = self.offsets.lock(); // Lock the offsets for safe access

        offsets.push(data.len());
        data.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn test() {
    let output1 = Box::new(Writer::new());
    let output2 = Box::new(Writer::new());
    let output3 = Box::new(Writer::new());

    logger::init([
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

