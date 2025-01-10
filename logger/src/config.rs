use std::{io::Write, path::PathBuf};

use hashbrown::HashMap;
use log::LevelFilter;

pub trait OutputStream: Write + Send + Sync {}

impl<T> OutputStream for T where T: Write + Send + Sync {}

pub enum Output {
    File(PathBuf),
    CustomStream(Box<dyn OutputStream>),
    Stdout,
    StdErr,
}

pub struct Config {
    pub output: Output,
    pub level: LevelFilter,
    pub filters: HashMap<String, LevelFilter>,
    pub colored: bool,
}

impl Config {
    pub fn output(mut self, output: Output) -> Self {
        self.output = output;

        self
    }

    pub fn level(mut self, level: LevelFilter) -> Self {
        self.level = level;
        self
    }

    pub fn filter(mut self, name: &str, level: LevelFilter) -> Self {
        // Make sure the user hasn't set the same filter twice, which im sure would be a mistake
        assert_eq!(self.filters.insert(name.to_string(), level), None);

        self
    }

    pub fn filters(mut self, filters: &[(&str, LevelFilter)]) -> Self{
        for (name, filter) in filters.iter(){
            self = self.filter(name,*filter);
        }
        self
    }

    pub fn colored(mut self, colored: bool) -> Self {
        self.colored = colored;
        self
    }
}

impl Output {
    pub(crate) fn into_stream(self) -> Box<dyn OutputStream> {
        match self {
            Self::File(path) => Box::new(
                std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                    .unwrap(),
            ),
            Self::CustomStream(stream) => stream,
            Self::Stdout => Box::new(std::io::stdout()),
            Self::StdErr => Box::new(std::io::stderr()),
        }
    }
}

impl From<Config> for Vec<Config> {
    fn from(cfg: Config) -> Self {
        vec![cfg]
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            output: Output::Stdout,
            level: LevelFilter::Trace,
            filters: HashMap::new(),
            colored: false,
        }
    }
}
