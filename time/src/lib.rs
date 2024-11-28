/// A delay for game animations (e.g., animation frames).
///
/// # Examples
///
/// ```
/// use time::DTDelay;
///
/// let mut delay = DTDelay::new(5.0);
/// println!("{:?}", delay); // DTDelay { starting_timeout_s: 5.0, timeout_s: 5.0 }
///
/// delay.update(1.0);
/// println!("{:?}", delay); // DTDelay { starting_timeout_s: 5.0, timeout_s: 4.0 }
///
/// println!("Fraction: {}", delay.fraction()); // Fraction: 0.8
///
/// println!("Ended: {}", delay.ended()); // Ended: false
///
/// delay.update(4.0);
/// println!("{:?}", delay); // DTDelay { starting_timeout_s: 5.0, timeout_s: 0.0 }
///
/// println!("Ended: {}", delay.ended()); // Ended: true
///
/// delay.update(1.0);
/// println!("{:?}", delay); // DTDelay { starting_timeout_s: 5.0, timeout_s: -1.0 }
///
/// println!("Time since ended: {}", delay.time_since_ended()); // Time since ended: 1.0
/// ```

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(from = "f64")
)]
#[derive(derivative::Derivative, Default, Copy, Debug, Clone)]
#[derivative(PartialEq)]
pub struct DTDelay {
    starting_timeout_s: f64, // Initial timeout value in seconds

    #[derivative(PartialEq = "ignore")]
    // Ignore this field when comparing two DTDelay instances for equality
    // because it is too precise to reliably compare.
    timeout_s: f64, // Current timeout value in seconds; set to the desired time, then decrease with delta time until it reaches 0
}

impl DTDelay {
    /// Creates a new `DTDelay` with the specified timeout in seconds.
    pub fn new(timeout_s: f64) -> Self {
        Self {
            timeout_s,
            starting_timeout_s: timeout_s,
        }
    }

    /// Restarts the `DTDelay`, resetting the timeout to the initial value.
    pub fn restart(&mut self) {
        *self = Self::new(self.starting_timeout_s)
    }

    /// Restart the `DTDelay` with a custom timeline.
    ///
    /// This is useful for better consistency over low framerate.
    ///
    /// Often paired with [`DTDelay::time_since_ended`]
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset to subtract from the timeout value.
    pub fn restart_custom_timeline(&mut self, offset: f64) {
        *self = Self {
            starting_timeout_s: self.starting_timeout_s,
            timeout_s: self.starting_timeout_s - offset,
        }
    }

    /// Updates the timeout by decreasing it with the given delta time.
    /// # Note
    ///
    /// The delta time has to be in seconds
    ///
    /// # Arguments
    ///
    /// * `dt` - The delta time to subtract from the timeout.
    pub fn update(&mut self, dt: f64) {
        self.timeout_s -= dt;
    }

    /// Returns the fraction of time remaining (0.0 < fraction < 1.0).
    pub fn fraction(&self) -> f64 {
        self.timeout_s / self.starting_timeout_s
    }

    /// Returns true if the timeout has ended (i.e., is less than or equal to 0).
    pub fn ended(&self) -> bool {
        self.timeout_s <= 0f64
    }

    /// Returns the time since the timeout ended.
    /// If negative, the delay has not finished yet.
    ///
    /// This is generally used as offset for [`DTDelay::restart_custom_timeline`]
    pub fn time_since_ended(&self) -> f64 {
        self.timeout_s * -1.0
    }
}

impl From<f64> for DTDelay {
    /// Allows creating a `DTDelay` instance from a `f64` value.
    ///
    /// # Arguments
    ///
    /// * `timeout_s` - The initial timeout value in seconds.
    ///
    /// # Returns
    ///
    /// A `DTDelay` instance with the given timeout.
    fn from(timeout_s: f64) -> DTDelay {
        DTDelay::new(timeout_s)
    }
}

/// Like a std::time::Instant, but you can stop it to read later
///
/// # Examples
///
/// ```
/// use time::Stopwatch;
///
/// let mut stopwatch = Stopwatch::start_new();
///
/// std::thread::sleep(std::time::Duration::from_secs_f32(1.5));
///
/// println!("{}", time::format(stopwatch.read(), 1)); // 1.5s
///
/// stopwatch.stop();
///
/// std::thread::sleep(std::time::Duration::from_secs(1));
///
/// println!("{}", time::format(stopwatch.read(), 1)); // 1.5s
/// ```

#[derive(Debug, Clone)]
/// Measure the time between the .start and .stop functions, can be read later
pub enum Stopwatch {
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
        write!(f, "{}", format(self.read(), -1))
    }
}

/// Formats a `std::time::Duration` into a human-readable string with specified precision.
///
/// # Parameters
///
/// - `duration`: The `std::time::Duration` to format.
/// - `prec`: The precision specifying the number of time units to display.
///   - If `prec` is a positive integer, it limits the output to that many units, starting from the largest unit (e.g., years, weeks, days, etc.).
///   - If `prec` is -1, all non-zero units will be displayed.
///
/// # Returns
///
/// A `String` representing the formatted duration with the specified precision.
///
/// # Examples
///
/// ```
/// let duration = std::time::Duration::new(80000, 0); // 80,000 seconds
///
/// // Displaying only the largest unit (precision = 1)
/// assert_eq!(time::format(duration, 1), "22h");
///
/// // Displaying the two largest units (precision = 2)
/// assert_eq!(time::format(duration, 2), "22h 13m");
///
/// // Displaying all non-zero units (precision = -1)
/// assert_eq!(time::format(duration, -1), "22h 13m 20s");
///
/// // Duration of 0 seconds
/// let duration_zero = std::time::Duration::new(0, 0);
/// assert_eq!(time::format(duration_zero, -1), "0ns");
/// ```
///
/// # Notes
///
/// - The function handles up to years, weeks, days, hours, minutes, seconds, milliseconds, microseconds, and nanoseconds.
/// - Units with zero value are omitted in the output.
/// - The precision parameter affects the number of units shown, starting from the largest available unit.
///
/// # Units Conversion
///
/// - 1 year = 365 days
/// - 1 week = 7 days
/// - 1 day = 24 hours
/// - 1 hour = 60 minutes
/// - 1 minute = 60 seconds
/// - 1 second = 1,000 milliseconds
/// - 1 millisecond = 1,000 microseconds
/// - 1 microsecond = 1,000 nanoseconds
///
/// # Panics
///
/// This function does not panic.
pub fn format(duration: std::time::Duration, mut prec: i8) -> String {
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
        return format!("{:.0}ns", total_nanos.floor());
    }

    let mut remaining_nanos = total_nanos;
    let mut formatted_duration = String::new();

    if remaining_nanos >= NANOS_IN_YEAR && prec != 0 {
        prec -= 1;
        let years = remaining_nanos / NANOS_IN_YEAR;
        formatted_duration.push_str(&format!("{:.0}y ", years.floor()));
        remaining_nanos %= NANOS_IN_YEAR;
    }

    if remaining_nanos >= NANOS_IN_WEEK && prec != 0 {
        prec -= 1;
        let weeks = remaining_nanos / NANOS_IN_WEEK;
        formatted_duration.push_str(&format!("{:.0}w ", weeks.floor()));
        remaining_nanos %= NANOS_IN_WEEK;
    }

    if remaining_nanos >= NANOS_IN_DAY && prec != 0 {
        prec -= 1;
        let days = remaining_nanos / NANOS_IN_DAY;
        formatted_duration.push_str(&format!("{:.0}d ", days.floor()));
        remaining_nanos %= NANOS_IN_DAY;
    }

    if remaining_nanos >= NANOS_IN_HOUR && prec != 0 {
        prec -= 1;
        let hours = remaining_nanos / NANOS_IN_HOUR;
        formatted_duration.push_str(&format!("{:.0}h ", hours.floor()));
        remaining_nanos %= NANOS_IN_HOUR;
    }

    if remaining_nanos >= NANOS_IN_MINUTE && prec != 0 {
        prec -= 1;
        let minutes = remaining_nanos / NANOS_IN_MINUTE;
        formatted_duration.push_str(&format!("{:.0}m ", minutes.floor()));
        remaining_nanos %= NANOS_IN_MINUTE;
    }

    if remaining_nanos >= NANOS_IN_SECOND && prec != 0 {
        prec -= 1;
        let seconds = remaining_nanos / NANOS_IN_SECOND;
        formatted_duration.push_str(&format!("{:.0}s ", seconds.floor()));
        remaining_nanos %= NANOS_IN_SECOND;
    }

    if remaining_nanos >= NANOS_IN_MILLISECOND && prec != 0 {
        prec -= 1;
        let milis = remaining_nanos / NANOS_IN_MILLISECOND;
        formatted_duration.push_str(&format!("{:.0}ms ", milis.floor()));
        remaining_nanos %= NANOS_IN_MILLISECOND;
    }

    if remaining_nanos >= NANOS_IN_MICROSECOND && prec != 0 {
        prec -= 1;
        let micro = remaining_nanos / NANOS_IN_MICROSECOND;
        formatted_duration.push_str(&format!("{:.0}µs ", micro.floor()));
        remaining_nanos %= NANOS_IN_MICROSECOND;
    }

    if remaining_nanos > 0.0 && prec != 0 {
        formatted_duration.push_str(&format!("{:.0}ns", remaining_nanos.floor()));
    }

    formatted_duration.trim().to_string()
}

/// Used to time the execution of a function with immutable parameters
/// # Example
/// ```
/// use time::timeit;
///
/// fn my_function(s: &str){
///     std::thread::sleep(std::time::Duration::from_secs(2));
///     println!("{s}");
/// }
///
/// let s = "Hi";
/// let (output, duration) = timeit( || my_function(s));
/// ```
pub fn timeit<F: FnOnce() -> T, T>(f: F) -> (T, std::time::Duration) {
    // The order of the output is important as it's also the order that it's computed
    // if you output (start.elapsed(), f()), the timer is stopped before the function actually starts
    // you'll need to compute f() before and store it in an ouput variable

    let start = std::time::Instant::now();
    // let output = f();
    (f(), start.elapsed())
}

/// Used to time the execution of a function with mutable parameters
/// # Example
/// ```
/// use time::timeit_mut;
///
/// fn my_mut_function(x: &mut i32){
///     std::thread::sleep(std::time::Duration::from_secs(2));
///     *x += 1
/// }
///
/// let mut y = 5;
/// let (output, duration) = timeit_mut( || my_mut_function(&mut y) );
/// ```
pub fn timeit_mut<F: FnMut() -> T, T>(mut f: F) -> (T, std::time::Duration) {
    let start = std::time::Instant::now();
    // let output = f();
    (f(), start.elapsed())
}

/// Used to time the execution of a function with mutable parameters
/// # Example
/// ```
/// use time::timeit_async;
///
/// async fn my_async_function() -> bool{
///     std::thread::sleep(std::time::Duration::from_secs(2));
///     true
/// }
///
/// async fn _main(){
///     let (output, duration) = timeit_async( || my_async_function() ).await;
/// }
///
/// ```
pub async fn timeit_async<F: std::future::Future<Output = T>, T>(
    f: impl FnOnce() -> F,
) -> (T, std::time::Duration) {
    let start = std::time::Instant::now();
    // let output = f().await;
    (f().await, start.elapsed())
}
