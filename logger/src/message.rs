
pub enum Message {
    Flush,
    Log {
        path: Option<String>,
        file: Option<String>,
        line: Option<u32>,
        content: String,
        level: log::Level,
    },
    Exit
}
