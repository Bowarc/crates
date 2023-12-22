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
    starting_timeout_s: f64,
    #[derivative(PartialEq = "ignore")]
    // i knowingly ignore the emplementation of PartialEq to this field beacause it is so precise that it
    // is impossible for two timeout to be equal
    timeout_s: f64, // Set it to the wanted time, (IN SECCONDS) then decrease it with given delta time, when reaches 0, it's done
}

impl DTDelay {
    pub fn new(timeout_s: f64) -> Self {
        Self {
            timeout_s,
            starting_timeout_s: timeout_s,
        }
    }
    pub fn new_custom_timeline(timeout_s: f64, offset: f64) -> Self {
        Self {
            timeout_s: timeout_s - offset,
            starting_timeout_s: timeout_s,
        }
    }
    pub fn restart(&mut self) {
        *self = Self::new(self.starting_timeout_s)
    }
    pub fn update(&mut self, dt: f64) {
        self.timeout_s -= dt;
    }
    pub fn fraction(&self) -> f64 {
        // has to be 0.0<frac<1.0
        self.timeout_s / self.starting_timeout_s
    }
    pub fn ended(&self) -> bool {
        self.timeout_s <= 0f64
    }
    pub fn time_since_ended(&self) -> f64 {
        self.timeout_s * -1. // if this is negative, the delay is not finished yet
    }
}

impl From<f64> for DTDelay {
    fn from(timeout_s: f64) -> DTDelay {
        DTDelay::new(timeout_s)
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
        write!(f, "{}", format(self.read()))
    }
}

pub fn format(duration: std::time::Duration) -> String {

    const NANOS_IN_MICROSECOND: f64 = 1_000.0;
    const NANOS_IN_MILLISECOND: f64 = 1_000_000.0;
    const NANOS_IN_SECOND: f64 = 1_000_000_000.0;
    const NANOS_IN_MINUTE: f64 = NANOS_IN_SECOND * 60.0;
    const NANOS_IN_HOUR: f64 = NANOS_IN_MINUTE * 60.0;
    const NANOS_IN_DAY: f64 = NANOS_IN_HOUR * 24.0;
    const NANOS_IN_WEEK: f64 = NANOS_IN_DAY * 7.0;
    const NANOS_IN_YEAR: f64 = NANOS_IN_DAY * 365.0;

    let total_nanos = duration.as_nanos() as f64;

    if total_nanos < 1.0 {
        return format!("{:.0}ns", total_nanos);
    }

    let mut remaining_nanos = total_nanos;
    let mut formatted_duration = String::new();

    if remaining_nanos >= NANOS_IN_YEAR {
        let years = remaining_nanos / NANOS_IN_YEAR;
        formatted_duration.push_str(&format!("{:.0}y ", years));
        remaining_nanos %= NANOS_IN_YEAR;
    }

    if remaining_nanos >= NANOS_IN_WEEK {
        let weeks = remaining_nanos / NANOS_IN_WEEK;
        formatted_duration.push_str(&format!("{:.0}w ", weeks));
        remaining_nanos %= NANOS_IN_WEEK;
    }

    if remaining_nanos >= NANOS_IN_DAY {
        let days = remaining_nanos / NANOS_IN_DAY;
        formatted_duration.push_str(&format!("{:.0}d ", days));
        remaining_nanos %= NANOS_IN_DAY;
    }

    if remaining_nanos >= NANOS_IN_HOUR {
        let hours = remaining_nanos / NANOS_IN_HOUR;
        formatted_duration.push_str(&format!("{:.0}h ", hours));
        remaining_nanos %= NANOS_IN_HOUR;
    }

    if remaining_nanos >= NANOS_IN_MINUTE {
        let minutes = remaining_nanos / NANOS_IN_MINUTE;
        formatted_duration.push_str(&format!("{:.0}m ", minutes));
        remaining_nanos %= NANOS_IN_MINUTE;
    }

    if remaining_nanos >= NANOS_IN_SECOND {
        let seconds = remaining_nanos / NANOS_IN_SECOND;
        formatted_duration.push_str(&format!("{:.0}s ", seconds));
        remaining_nanos %= NANOS_IN_SECOND;
    }

    if remaining_nanos >= NANOS_IN_MILLISECOND{
        let milis = remaining_nanos / NANOS_IN_MILLISECOND;
        formatted_duration.push_str(&format!("{:.0}ms ", milis));
        remaining_nanos %= NANOS_IN_MILLISECOND;
    }

    if remaining_nanos >= NANOS_IN_MICROSECOND{
        let micro = remaining_nanos / NANOS_IN_MICROSECOND;
        formatted_duration.push_str(&format!("{:.0}µs ", micro));
        remaining_nanos %= NANOS_IN_MICROSECOND;
    }

    if remaining_nanos > 0.0 {
        formatted_duration.push_str(&format!("{:.0}ns", remaining_nanos));
    }

    formatted_duration.trim().to_string()
}

pub fn timeit<F: Fn() -> T, T>(f: F) -> (T, std::time::Duration) {
    //! Used to time the execution of a function with immutable parameters
    //! # Example
    //! ```
    //! let (output, duration) = timeit( || my_function() )
    //! ```

    // The order of the output is important as it's also the order that it's computed
    // if you output (start.elapsed(), f()), the timer is stopped before the function actually starts
    // you'll need to compute f() before and store it in an ouput variable

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

    let start = std::time::Instant::now();
    // let output = f();
    (f(), start.elapsed())
}

pub async fn timeit_async<F: std::future::Future<Output = T>, T>(f: F) -> (T, std::time::Duration) {
    //! Used to time the execution of a function with mutable parameters
    //! # Example
    //! ```
    //! let (output, duration) = timeit_async( my_future )
    //!
    //! ```

    let start = std::time::Instant::now();
    // let output = f();
    (f.await, start.elapsed())
}
