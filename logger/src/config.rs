use chrono::Duration;
use hashbrown::HashMap;
use log::LevelFilter;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    str::FromStr,
};
use thiserror::Error;

pub trait OutputStream: Write + Send + Sync {}

impl<T> OutputStream for T where T: Write + Send + Sync {}

#[derive(Debug, Error, PartialEq)]
pub enum ConfigError {
    #[error(transparent)]
    InvalidOutput(#[from] InvalidOutputError),
}

#[derive(Debug, Error, PartialEq)]
pub enum InvalidOutputError {
    #[error("Incorect file path, please make sure the path ends up with a file name")]
    NotAFile,
    #[error("Incorect file path, please make sure the parent directories exists")]
    DirectoryDoesNotExist,
    #[error("Incorect file path, the given directory is read only")]
    ReadOnlyDirectory,
}

pub enum Output {
    File(PathBuf),
    TimedFile { path: PathBuf, interval: Duration },
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
    pub fn try_output(mut self, output: impl Into<Output>) -> Result<Self, ConfigError> {
        let output = output.into();

        // Make sure the path is correct
        'ensure_correct: {
            let mut path = match &output {
                Output::File(pathbuf) => pathbuf.clone(),
                Output::TimedFile { path, .. } => path.clone(),
                _ => break 'ensure_correct,
            };

            // Make sure the path is a file, even if it's not yet created
            if path.file_name().is_none()
                || path.to_str().unwrap().ends_with('/')
                || path.to_str().unwrap().ends_with('\\')
            {
                Err(InvalidOutputError::NotAFile)?
            }

            path.pop();

            // If the path was only a filename, due to path.pop, it is now empty, we therefore add the current directory to it
            if path.to_str().unwrap().is_empty() {
                path.push(".");
            }

            // Make sure the path is a directory and exists
            if !path.is_dir() || !path.exists() {
                Err(InvalidOutputError::DirectoryDoesNotExist)?
            }

            // Check write access to the directory
            let Ok(metadata) = fs::metadata(&path) else {
                eprintln!("[WARN] Could not query metadata for path: {path:?}, therefore could not make sure we have write access");
                break 'ensure_correct;
            };
            if metadata.permissions().readonly() {
                Err(InvalidOutputError::ReadOnlyDirectory)?
            }
        }

        self.output = output;
        Ok(self)
    }

    pub fn output(self, output: impl Into<Output>) -> Self {
        self.try_output(output).unwrap()
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

    pub fn filters(mut self, filters: &[(&str, LevelFilter)]) -> Self {
        for (name, filter) in filters.iter() {
            self = self.filter(name, *filter);
        }
        self
    }

    pub fn colored(mut self, colored: bool) -> Self {
        self.colored = colored;
        self
    }
}

impl Output {
    pub fn new_timed_file(
        path: impl Into<PathBuf>,
        interval: impl Into<std::time::Duration>,
    ) -> Self {
        Self::TimedFile {
            path: path.into(),
            interval: Duration::from_std(interval.into()).unwrap(),
        }
    }
    pub(crate) fn into_stream(self) -> Box<dyn OutputStream> {
        match self {
            Self::File(path) => Box::new(
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                    .unwrap(),
            ),
            Self::TimedFile { path, interval } => {
                Box::new(crate::timed_file::TimedFile::new(path, interval))
            }
            Self::CustomStream(stream) => stream,
            Self::Stdout => Box::new(std::io::stdout()),
            Self::StdErr => Box::new(std::io::stderr()),
        }
    }
}

impl<T: OutputStream + 'static> From<Box<T>> for Output {
    fn from(steam: Box<T>) -> Self {
        Output::CustomStream(steam)
    }
}

impl From<PathBuf> for Output {
    fn from(path: PathBuf) -> Self {
        Output::File(path)
    }
}

impl From<&str> for Output {
    fn from(path: &str) -> Self {
        Output::File(PathBuf::from_str(path).unwrap())
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

// Originally used for tests, i don't see the point of actually implementing thoses
// impl std::fmt::Debug for Output {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match &self {
//             Output::File(pathbuf) => write!(f, "Output::File(path: {pathbuf:?})"),
//             Output::TimedFile { path, interval } => {
//                 write!(f, "Output::TimedFile(path: {path:?}, interval: {interval})")
//             }
//             Output::CustomStream(..) => write!(f, "Output::CustomStream(..)"),
//             Output::Stdout => write!(f, "Output::Stdout"),
//             Output::StdErr => write!(f, "Output::StdErr"),
//         }
//     }
// }

// impl std::fmt::Debug for Config {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Config")
//             .field("output", &self.output)
//             .field("level", &self.level)
//             .field("filters", &self.filters)
//             .field("colored", &self.colored)
//             .finish()
//     }
// }

// impl std::cmp::PartialEq for Output {
//     fn eq(&self, other: &Self) -> bool {
//         std::mem::discriminant(self) == std::mem::discriminant(other)
//     }
// }
