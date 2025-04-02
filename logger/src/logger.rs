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

        let most_accurate_filter =
            get_most_accurate_filter(source, &self.filters).unwrap_or(self.level);

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

fn get_most_accurate_filter(
    source: &str,
    filters: &[(String, LevelFilter)],
) -> Option<log::LevelFilter> {
    // The goal here, is to get the distance from the last position of the match to the end of the searched string
    let get_index =
        |s1: &str, s2: &str| -> Option<usize> { s1.find(s2).map(|i| s1.len() - s2.len() + i) };

    let mut possible_filters = filters
        .iter()
        .flat_map(|(filter, level)| Some((level, get_index(source, filter)?)))
        .collect::<Vec<_>>();

    if possible_filters.is_empty() {
        return None;
    }

    // Get the filter with the lowest number
    possible_filters.sort_by_cached_key(|(_s, index)| *index);

    // println!("{possible_filters:?}");
    Some(*possible_filters.first().unwrap().0)
}

// This one works too, but i think it's slower
// fn get_most_accurate_filter(
//     source: &str,
//     filters: &[(String, LevelFilter)],
// ) -> Option<log::LevelFilter> {
//     let mut possible_filters = filters
//         .iter()
//         .filter(|(filter, _)| source.contains(filter))
//         .collect::<Vec<_>>();
//
//     possible_filters.sort_by(|&(a, _), &(b, _)| {
//         let a_index = source.find(a).unwrap();
//         let b_index = source.find(b).unwrap();
//
//         let index_cmp = a_index.cmp(&b_index);
//         if index_cmp != std::cmp::Ordering::Equal {
//             return index_cmp;
//         }
//
//         b.len().cmp(&a.len()) // Reverse order for longer strings
//     });
//
//     println!("{possible_filters:?}");
//
//     possible_filters.first().map(|(_, level)| level).cloned()
// }

#[test]
fn test() {
    use log::LevelFilter;
    let filters = vec![
        ("Test".to_string(), LevelFilter::Off),
        ("Test::a".to_string(), LevelFilter::Trace),
        ("Te".to_string(), LevelFilter::Debug),
        ("lib::long_module_name".to_string(), LevelFilter::Info),
        ("lib".to_string(), LevelFilter::Warn),
    ];
    assert_eq!(
        get_most_accurate_filter("Test::", &filters),
        Some(LevelFilter::Off)
    );
    assert_eq!(
        get_most_accurate_filter("Test::a", &filters),
        Some(LevelFilter::Trace)
    );
    assert_eq!(
        get_most_accurate_filter("Teta::", &filters),
        Some(LevelFilter::Debug)
    );
    assert_eq!(
        get_most_accurate_filter("mylib", &filters),
        Some(LevelFilter::Warn)
    );
    assert_eq!(
        get_most_accurate_filter("lib", &filters),
        Some(LevelFilter::Warn)
    );
    assert_eq!(
        get_most_accurate_filter("lib::", &filters),
        Some(LevelFilter::Warn)
    );
}
