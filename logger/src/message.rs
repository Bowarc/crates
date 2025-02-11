
pub enum Message {
    Flush,
    Log {
        source: String,
        line: Option<u32>,
        content: String,
        level: log::Level,
    },
    Exit
}
