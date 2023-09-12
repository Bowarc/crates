#[derive(Debug)]
pub struct LoggerConfig {
    global_level: log::LevelFilter,
    filters: Vec<(String, log::LevelFilter)>,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            global_level: log::LevelFilter::Trace,
            filters: vec![],
        }
    }
}

impl LoggerConfig {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_level(mut self, level: log::LevelFilter) -> Self {
        self.global_level = level;
        self
    }
    pub fn add_filter(mut self, name: &str, level: log::LevelFilter) -> Self {
        self.filters.push((name.to_string(), level));
        self
    }
}

fn colorise(message: String, level: log::Level) -> colored::ColoredString {
    use colored::Colorize as _;
    // #[cfg(not(debug_assertions))]
    // #[cfg(debug_assertions)]
    match level {
        log::Level::Trace => message.normal(),
        log::Level::Debug => message.cyan(),
        log::Level::Info => message.green(),
        log::Level::Warn => message.yellow(),
        log::Level::Error => message.red(),
        // _ => message.normal(),
    }
}

/*
    Note


    .file = path bcp trop long du fichier qui a demandé le log

    .target = namespace du fichier

*/

fn generate_file_name(record: &log::Record) -> String {
    /*
        Extremely boring and very unstable but i don't even care anymore
    */
    let final_file_name = record
        .file()
        .unwrap_or("Unknown file")
        .split('\\')
        .last()
        .unwrap()
        .split('/')
        .last()
        .unwrap();

    let module_path = record.module_path().unwrap();

    if module_path.split("::").last().unwrap() == final_file_name.replace(".rs", "") {
        format!("{module_path}.rs")
    } else {
        format!("{module_path}::{final_file_name}")
    }
}

pub fn init(config: LoggerConfig, log_file_opt: Option<&str>) {
    let mut builder = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "╭[{time} {level} {file_path}:{line_nbr}]\n╰❯{message}",
                time = chrono::Local::now().format("%H:%M:%S%.3f"),
                level = colorise(record.level().to_string(), record.level()),
                file_path = generate_file_name(record),
                line_nbr = record
                    .line()
                    .map(|l| l.to_string())
                    .unwrap_or_else(|| "?".to_string()),
                message = colorise(message.to_string(), record.level())
            ));
        })
        .level(config.global_level)
        .chain(std::io::stdout());
    if let Some(log_file) = log_file_opt {
        builder = builder.chain(fern::log_file(log_file).unwrap());
    }
    for filter in config.filters.iter() {
        builder = builder.level_for(filter.0.clone(), filter.1);
    }
    builder.apply().unwrap();

    log_panics::Config::new()
        .backtrace_mode(log_panics::BacktraceMode::Resolved)
        .install_panic_hook()
}

pub fn test() {
    use log::{debug, error, info, trace, warn};

    // trace!("This is Trace level"); // target: "custom_target",
    // debug!("This is Debug level");
    // info!("This is Info level");
    // warn!("This is Warn level");
    // error!("This is Error level");

    for i in 0..26 {
        trace!("loading: {}%, very verbose debbuging information", 4 * i);
        if 5 == i {
            debug!("this is taking so long... boooring!");
        } else if 10 == i {
            debug!("still alive! yay!");
        } else if 13 == i {
            info!("halfway there!");
        } else if 16 == i {
            debug!("*scratches nose*");
            warn!("nose is itching, continuing anyways");
        } else if 20 == i {
            debug!("uh oh");
            warn!(">nose itching intensifies");
            error!("HATCHOOO!");
            debug!("encountered minor problem, trying to recover");
            info!("gesundheit");
            debug!("recovered from minor problem, continuing");
        } else if 25 == i {
            info!("successfully loaded nothing");
            info!("have a good time!");
        }
    }
}
