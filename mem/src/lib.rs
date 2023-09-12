pub fn display_bytes(bytes: usize) -> String {
    const KB: usize = 1024;
    const MB: usize = KB * 1024;
    const GB: usize = MB * 1024;
    const TB: usize = GB * 1024;
    const PB: usize = TB * 1024;

    if bytes < KB {
        format!("{}B", bytes)
    } else if bytes < MB {
        format!("{:.2}KB", bytes as f64 / KB as f64)
    } else if bytes < GB {
        format!("{:.2}MB", bytes as f64 / MB as f64)
    } else if bytes < TB {
        format!("{:.2}GB", bytes as f64 / GB as f64)
    } else if bytes < PB {
        format!("{:.2}TB", bytes as f64 / TB as f64)
    } else {
        format!("{:.2}PB", bytes as f64 / PB as f64)
    }
}
