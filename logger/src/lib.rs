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
    fn apply(&self, mut dispatch: fern::Dispatch) -> fern::Dispatch {
        for filter in &self.filters {
            dispatch = dispatch.level_for(filter.0.clone(), filter.1);
        }
        dispatch = dispatch.level(self.global_level);
        dispatch
    }
}

fn colorise(message: String, level: log::Level) -> colored::ColoredString {
    use colored::Colorize as _;
    match level {
        log::Level::Trace => message.normal(),
        log::Level::Debug => message.cyan(),
        log::Level::Info => message.green(),
        log::Level::Warn => message.yellow(),
        log::Level::Error => message.red(),
        // _ => message.normal(),
    }
}

fn generate_file_name(record: &log::Record) -> String {
    /*
        Note
        .file is the wayyy too long path of the file that called the log
        .target is the namespace of that file

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

fn generate_message(message: &std::fmt::Arguments, record: &log::Record, color: bool) -> String {
    format!(
        "[{time} {level} {file_path}:{line_nbr}]\n{message}",
        time = chrono::Local::now().format("%H:%M:%S%.3f"),
        level = if color {
            format!("{}", colorise(record.level().to_string(), record.level()))
        } else {
            record.level().to_string()
        },
        file_path = generate_file_name(record),
        line_nbr = record
            .line()
            .map(|l| l.to_string())
            .unwrap_or_else(|| "?".to_string()),
        message = if color {
            format!("{}", colorise(message.to_string(), record.level()))
        } else {
            message.to_string()
        }
    )
}

pub fn init(config: LoggerConfig, log_file_opt: Option<&str>) {
    /*
        Here we make two dispatchers
            One that use colored output for stdout,
            And the seccond one without colors, for a given file
            (because it's anoying to have a file with ASCII escape codes)
    */

    let mut dispatch = fern::Dispatch::new().chain({
        let mut stdout_dispatch = fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!("{}", generate_message(message, record, true)));
            })
            .level(config.global_level)
            .chain(std::io::stdout());

        stdout_dispatch = config.apply(stdout_dispatch);
        stdout_dispatch
    });

    if let Some(file_path) = log_file_opt {
        dispatch = dispatch.chain({
            let mut l = fern::Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!("{}", generate_message(message, record, false)));
                })
                .chain(fern::log_file(file_path).unwrap());
            l = config.apply(l);
            l
        })
    }

    dispatch.apply().unwrap();

    log_panics::Config::new()
        .backtrace_mode(log_panics::BacktraceMode::Resolved)
        .install_panic_hook()
}

pub fn test() {
    use log::{debug, error, info, trace, warn};

    trace!("This is Trace level"); // target: "custom_target",
    debug!("This is Debug level");
    info!("This is Info level\nThis is Info level Multiline\nThis is Info level Multiline\nThis is Info level Multiline\nThis is Info level Multiline\n");
    warn!("This is Warn level");
    error!("This is Error level");

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
