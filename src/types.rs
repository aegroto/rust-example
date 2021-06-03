use std::fmt;

pub enum AnalyticsMessageData {
    HiddenValuesFrequencies((usize, usize)),
    Alterations(usize),
    Stop(),
}

pub trait Alterable {
    fn is_altered(&self) -> bool;
}

pub struct Entry {
    pub angle: u8,
    pub new_angle: u8,
    pub hidden_value: u8,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            self.angle, self.new_angle, self.hidden_value
        )
    }
}

impl Alterable for Entry {
    fn is_altered(&self) -> bool {
        self.angle != self.new_angle
    }
}
