use {
    crate::Config,
    colored::ColoredString,
    log::{Level, LevelFilter},
    std::io::Write,
};

pub struct Logger {
    output: parking_lot::Mutex<Box<dyn super::config::OutputStream>>,
    level: LevelFilter,
    filters: Vec<(String, LevelFilter)>,
    colored: bool,
}

fn color(message: &str, level: &Level) -> ColoredString {
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

impl Logger {
    pub fn from_cfg(cfg: Config) -> Self {
        Self {
            output: parking_lot::Mutex::new(cfg.output.into_stream()),
            filters: {
                let mut filters = cfg
                    .filters
                    .into_iter()
                    .collect::<Vec<(String, LevelFilter)>>();
                filters.sort_unstable_by(|(name1, _level1), (name2, _level2)| name1.cmp(name2));
                filters
            },
            level: cfg.level,
            colored: cfg.colored,
        }
    }
    pub fn gen_message(
        source: &String,
        line: Option<u32>,
        content: &String,
        level: &Level,
        colored: bool,
    ) -> String {
        format!(
            "[{time} {level} {path}:{line_nbr}] {message}",
            time = chrono::Local::now().format("%H:%M:%S%.3f"),
            level = if colored {
                color(level.as_str(), level).to_string()
            } else {
                level.to_string()
            },
            path = source,
            // file = file_name(file.unwrap_or("Unknown file")),
            line_nbr = line
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or("?!?".to_string()),
            message = if colored {
                color(content, level).to_string()
            } else {
                content.to_string()
            }
        )
    }

    pub fn log(&self, source: &String, line: Option<u32>, content: &String, level: &Level) {
        // let last_filter = self
        //     .filters
        //     .iter()
        //     .filter(|(k, _v)| source.contains(k))
        //     .last()
        //     .map(|(_k, v)| v)
        //     .unwrap_or(&self.level);

        let most_accurate_filter = 'a:  {
            let mut possible_filters = self
                .filters
                .iter()
                .flat_map(|(f, level)| Some((level, source.find(f)?)))
                .collect::<Vec<_>>();

            if possible_filters.is_empty() {
                break 'a self.level;
            }

            possible_filters.sort_unstable_by_key(|(_s, index)| *index);

            *possible_filters.first().unwrap().0
        };
        // println!("{most_accurate_filter:?}");

        if &most_accurate_filter < level {
            // println!("Skipped '{content}' from {source}, because the level '{level}' was too low");
            return;
        }

        writeln!(
            self.output.lock(),
            "{}",
            Self::gen_message(source, line, content, level, self.colored)
        )
        .unwrap();
    }

    pub fn flush(&self) {}
}
