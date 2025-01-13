pub enum Prefix {
    Decimal,
    Binary,
}

impl Prefix{
    const DECIMAL_UNITS: &[&str] = &["b", "Kb", "Mb", "Gb", "Tb", "Pb", "Eb", "Zb", "Yb"];
    const BINARY_UNITS: &[&str] = &["b", "Kib", "Mib", "Gib", "Tib", "Pib", "Eib", "Zib", "Yib"];

    const fn units(&self) -> &[&str]{
        match self{
            Self::Decimal => Self::DECIMAL_UNITS,
            Self::Binary => Self::BINARY_UNITS,
        }
    }
}

impl From<&Prefix> for f64 {
    fn from(prefix: &Prefix) -> f64 {
        match prefix {
            Prefix::Decimal => 1000.,
            Prefix::Binary => 1024.,
        }
    }
}

pub fn format(bytes: u64, prefix: &Prefix) -> String {
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let units = prefix.units();
    let prefix = f64::from(prefix);
    
    let mut size = bytes as f64;
    let mut index = 0;

    while size >= prefix && index < units.len() - 1 {
        size /= prefix;
        index += 1;
    }

    format!("{:.2} {}", size, units[index])
}

#[inline(always)]
pub fn format_decimal(bytes: u64) -> String{
    format(bytes, &Prefix::Decimal)
}

#[inline(always)]
pub fn format_binary(bytes: u64) -> String{
    format(bytes, &Prefix::Binary)
}
