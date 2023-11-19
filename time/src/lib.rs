#[derive(PartialEq, Eq)]
pub enum DelayState<T> {
    // Timeline: --------------------------------
    //       delay ended here|    |but `ended()` is called here
    // We return the time between the two bars
    Done(T), //Time since done
    Running,
}

#[derive(derivative::Derivative, Default, Copy, Debug, Clone, serde::Deserialize)]
#[derivative(PartialEq)]
#[serde(from = "f64")]
/// Mostly used in animations, The Delay is good for waiting (ex: animations frames)
pub struct DTDelay {
    starting_timeout: f64,
    #[derivative(PartialEq = "ignore")]
    // i knowingly ignore the emplementation of PartialEq to this field beacause it is so precise that it
    // is impossible for two timeout to be equal
    timeout: f64, // Set it to the wanted time, (IN SECCONDS) then decrease it with given delta time, when reaches 0, it's done
}

impl DTDelay {
    pub fn new(timeout: f64) -> Self {
        Self {
            timeout,
            starting_timeout: timeout,
        }
    }
    pub fn new_custom_timeline(timeout: f64, offset: f64) -> Self {
        Self {
            timeout: timeout - offset,
            starting_timeout: timeout,
        }
    }
    pub fn restart(&mut self) {
        *self = Self::new(self.starting_timeout)
    }
    pub fn update(&mut self, dt: f64) {
        self.timeout -= dt;
    }
    pub fn fraction(&self) -> f64 {
        // has to be 0.0<frac<1.0
        self.timeout / self.starting_timeout
    }
    pub fn ended(&self) -> bool {
        self.timeout <= 0f64
    }
    pub fn time_since_ended(&self) -> f64 {
        self.timeout * -1. // if this is negative, the delay is not finished yet
    }
}

impl From<f64> for DTDelay {
    fn from(timeout: f64) -> DTDelay {
        DTDelay::new(timeout)
    }
}

#[derive(Debug, Clone)]
/// Measure the time between the .start and .stop functions, can be read later
pub enum Stopwatch {
    // Ps i used an enum as it best fits the use to me, + it's globally smaller as it re-uses the memory if the other state for the curent one
    Running {
        start_time: std::time::Instant,
    },
    Paused {
        paused_since: std::time::Instant,
        runtime: std::time::Duration,
    },
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::Paused {
            paused_since: std::time::Instant::now(),
            runtime: std::time::Duration::from_secs(0),
        }
    }
}

impl Stopwatch {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn start_new() -> Self {
        Self::Running {
            start_time: std::time::Instant::now(),
        }
    }
    pub fn is_running(&self) -> bool {
        matches![self, Stopwatch::Running { .. }]
    }
    pub fn is_stopped(&self) -> bool {
        !self.is_running()
    }
    pub fn start(&mut self) {
        *self = Stopwatch::start_new();
    }
    pub fn stop(&mut self) {
        if let Self::Running { start_time } = self {
            *self = Stopwatch::Paused {
                paused_since: std::time::Instant::now(),
                runtime: start_time.elapsed(),
            }
        }
    }
    pub fn read(&self) -> std::time::Duration {
        match self {
            Stopwatch::Running { start_time } => start_time.elapsed(),
            Stopwatch::Paused { runtime, .. } => *runtime,
        }
    }
}

impl std::fmt::Display for Stopwatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", display_duration(self.read()))
    }
}

// pub fn display_duration(d: std::time::Duration, separator: &str) -> String {
//     let mut value: f64 = d.as_nanos() as f64;
//     // debug!("d:{:?}", d);
//     // if nanos == 0 {}
//     // debug!("nbr: {}", nbr);

//     let units: Vec<&str> = vec!["ns", "µs", "ms", "s"];
//     let mut name_index = 0;

//     while value >= 1_000. {
//         if name_index < units.len() - 1 {
//             value /= 1_000.;
//             name_index += 1
//         } else {
//             break;
//         }
//     }
//     format!("{:.2}{}{}", value, separator, units[name_index])
// }

pub fn display_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    let nanos = duration.subsec_nanos();

    if secs == 0 {
        if nanos < 1_000 {
            return format!("{}ns", nanos);
        } else if nanos < 1_000_000 {
            return format!("{:.2}µs", nanos as f64 / 1_000.0);
        } else {
            return format!("{:.2}ms", nanos as f64 / 1_000_000.0);
        }
    }

    if secs < 60 {
        format!("{secs}s")
    } else if secs < 3_600 {
        let minutes = secs / 60;
        let seconds = secs % 60;
        format!("{minutes}m {seconds}s")
    } else if secs < 86_400 {
        let hours = secs / 3_600;
        let minutes = (secs % 3_600) / 60;
        format!("{hours}h {minutes}m")
    } else {
        let days = secs / 86_400;
        format!("{days}days")
    }
}

pub fn timeit<F: Fn() -> T, T>(f: F) -> (T, std::time::Duration) {
    //! Used to time the execution of a function with immutable parameters
    //! # Example
    //! ```
    //! let (output, duration) = timeit( || my_function() )
    //! ```
    let start = std::time::Instant::now();
    // let output = f();
    (f(), start.elapsed())
}

pub fn timeit_mut<F: FnMut() -> T, T>(mut f: F) -> (T, std::time::Duration) {
    //! Used to time the execution of a function with mutable parameters
    //! # Example
    //! ```
    //! let (output, duration) = timeit_mut( || my_function() )
    //! ```

    // the order of the output is important as it's also the order that it's cumputed
    // if you output (start.elapsed(), f()), the timer is stopped before the function actually starts
    // you'll need to compute f() before and store it in an ouput variable
    let start = std::time::Instant::now();
    // let output = f();
    (f(), start.elapsed())
}
