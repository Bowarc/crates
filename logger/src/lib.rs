mod config;
mod logger;
mod timed_file;

use std::sync::mpsc::{self, Receiver, Sender};

pub use config::{Config, Output, OutputStream};

struct ProxyLogger {
    sender: Sender<Message>,
}

impl log::Log for ProxyLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        // let enabled = self.cfgs.iter().any(|cfg| {
        //     cfg.filters.get(metadata.target()).unwrap_or(&cfg.level) >= &metadata.level()
        // });

        // enabled
        true
    }

    fn log(&self, record: &log::Record) {
        self.sender
            .send(Message::Log {
                path: record.module_path().map(ToString::to_string),
                file: record.file().map(ToString::to_string),
                line: record.line(),
                content: record.args().to_string(),
                level: record.level(),
            })
            .unwrap()
    }

    fn flush(&self) {
        self.sender.send(Message::Flush).unwrap()
    }
}

pub fn init(cfgs: impl Into<Vec<Config>>) {
    let cfgs = cfgs.into();

    let (sender, receiver) = mpsc::channel::<Message>();
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
        .unwrap();

    log::set_max_level(log::LevelFilter::Trace);
    log::set_boxed_logger(Box::new(ProxyLogger { sender })).unwrap();

    log_panics::Config::new()
        .backtrace_mode(log_panics::BacktraceMode::Resolved)
        .install_panic_hook()
}

enum Message {
    Flush,
    Log {
        path: Option<String>,
        file: Option<String>,
        line: Option<u32>,
        content: String,
        level: log::Level,
    },
}

fn logger(receiver: Receiver<Message>, mut loggers: Vec<logger::Logger>) {
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
    loop {
        let message = match receiver.recv() {
            Ok(message) => message,
            Err(why) => {
                println!("[Logger failled on: {why}]");
                return;
            }
        };

        match message {
            Message::Log {
                path,
                file,
                line,
                content,
                level,
            } => {
                let source = source(
                    path.as_deref().unwrap_or("Unknown module path"),
                    file.as_deref().unwrap_or("Unknown file"),
                );
                loggers
                    .iter_mut()
                    .for_each(|logger| logger.log(&source, line, &content, &level));
            }
            Message::Flush => loggers.iter_mut().for_each(|logger| logger.flush()),
        }
    }
}
