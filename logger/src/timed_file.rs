use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::PathBuf,
};

use chrono::{DateTime, Duration, Local};

pub struct TimedFile {
    interval: Duration,
    last_update: DateTime<Local>,
    inner_file: File,
    path: PathBuf,
}

impl TimedFile {
    pub fn new(path: PathBuf, interval: Duration) -> Self {
        let datetime = Local::now();

        let file = {
            let path = Self::gen_path(path.clone(), datetime).unwrap();

            match Self::open(path.clone()) {
                Ok(file) => file, 
                Err(why) => {
                    eprintln!("[ERROR] Failed to create file at {path:?} due to: {why}");
                    panic!();
                },
            }
        };

        Self {
            interval,
            last_update: datetime,
            inner_file: file,
            path,
        }
    }

    fn open(path: PathBuf) -> Result<File, io::Error> {
        OpenOptions::new().create(true).append(true).open(path)
    }

    fn gen_path(mut path: PathBuf, datetime: DateTime<Local>) -> Result<PathBuf, PathBuf> {
        let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
            eprintln!("[ERROR] Failed to generate new file name based on: {:?}", path);
            Err(path)?
        };

        let new_file_name = format!("{}-{file_name}", datetime.format("%Y-%m-%d_%H-%M-%S"));

        path.set_file_name(new_file_name);

        Ok(path)
    }
    fn update(&mut self) {
        let now = Local::now();

        if now.signed_duration_since(self.last_update) < self.interval {
            return;
        }

        self.last_update += self.interval
            * (now.signed_duration_since(self.last_update).num_seconds()
                / self.interval.num_seconds()) as i32;

        let Ok(new_path) = Self::gen_path(self.path.clone(), self.last_update) else {
            return;
        };

        let new_file = match Self::open(new_path) {
            Ok(new_file) => new_file,
            Err(why) => {
                log::error!("Failed to rotate TimedFile due to: {why}");
                return;
            }
        };

        self.inner_file = new_file;
    }
}

impl Write for TimedFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.update();
        self.inner_file.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner_file.flush()
    }
}
