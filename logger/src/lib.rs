mod config;
mod handle;
mod logger;
mod message;
mod timed_file;

use std::sync::mpsc::{self, Receiver, Sender};

pub use config::{Config, ConfigError, InvalidOutputError, Output, OutputStream};
pub use handle::LoggerThreadHandle;
use message::Message;

struct ProxyLogger {
    #[cfg(feature = "multithread")]
    sender: Sender<Message>,
    #[cfg(not(feature = "multithread"))]
    loggers: Vec<logger::Logger>,
}

impl log::Log for ProxyLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        #[inline(always)]
        fn source(path: &str, file: &str) -> String {
            // The patern String.split(..).last().unwrap() will never panic

            if file.ends_with(&format!("{}.rs", path.split("::").last().unwrap())) {
                format!("{path}.rs")
            } else {
                format!(
                    "{path}::{}",
                    file.split('\\').last().unwrap().split('/').last().unwrap()
                )
            }
        }

        let line = record.line();
        let content = record.args().to_string();
        let level = record.level();
        let source = source(
            record
                .module_path()
                .map(ToString::to_string)
                .as_deref()
                .unwrap_or("Unknown module path"),
            record
                .file()
                .map(ToString::to_string)
                .as_deref()
                .unwrap_or("Unknown file"),
        );

        #[cfg(not(feature = "multithread"))]
        self.loggers
            .iter()
            .for_each(|logger| logger.log(&source, line, &content, &level));

        #[cfg(feature = "multithread")]
        if let Err(e) = self.sender.send(Message::Log {
            source: source.clone(),
            line,
            content: content.clone(),
            level,
        }) {
            eprintln!(
                "[ERROR] Failed to send log record due to: {e}\n{}\n",
                logger::Logger::gen_message(&source, line, &content, &level, false)
            );
        }
    }

    fn flush(&self) {
        #[cfg(feature = "multithread")]
        self.sender.send(Message::Flush).unwrap();

        #[cfg(not(feature = "multithread"))]
        self.loggers.iter().for_each(|logger| logger.flush());
    }
}

#[cfg(feature = "multithread")]
#[must_use]
pub fn init(cfgs: impl Into<Vec<Config>>) -> LoggerThreadHandle {
    let cfgs = cfgs.into();

    let (sender, receiver) = mpsc::channel::<Message>();

    let handle = LoggerThreadHandle::new(
        sender.clone(),
        std::thread::Builder::new()
            .name("logger".to_string())
            .spawn(move || {
                logger(
                    receiver,
                    cfgs.into_iter()
                        .map(logger::Logger::from_cfg)
                        .collect::<Vec<logger::Logger>>(),
                );
            })
            .unwrap(),
    );

    log::set_max_level(log::LevelFilter::Trace);
    log::set_boxed_logger(Box::new(ProxyLogger { sender })).unwrap();

    #[cfg(feature = "panics")]
    log_panics::Config::new()
        .backtrace_mode(log_panics::BacktraceMode::Resolved)
        .install_panic_hook();

    handle
}

#[cfg(not(feature = "multithread"))]
#[must_use]
pub fn init(cfgs: impl Into<Vec<Config>>) {
    let cfgs = cfgs.into();

    log::set_max_level(log::LevelFilter::Trace);
    log::set_boxed_logger(Box::new(ProxyLogger {
        loggers: cfgs
            .into_iter()
            .map(logger::Logger::from_cfg)
            .collect::<Vec<logger::Logger>>(),
    }))
    .unwrap();

    #[cfg(feature = "panics")]
    log_panics::Config::new()
        .backtrace_mode(log_panics::BacktraceMode::Resolved)
        .install_panic_hook();
}

fn logger(receiver: Receiver<Message>, mut loggers: Vec<logger::Logger>) {
    loop {
        let message = match receiver.recv() {
            Ok(message) => message,
            Err(why) => {
                eprintln!("[Logger failled on: {why}]");
                return;
            }
        };
        match message {
            Message::Log {
                source,
                line,
                content,
                level,
            } => {
                loggers
                    .iter_mut()
                    .for_each(|logger| logger.log(&source, line, &content, &level));
            }
            Message::Flush => loggers.iter_mut().for_each(|logger| logger.flush()),
            Message::Exit => break,
        }
    }
}
