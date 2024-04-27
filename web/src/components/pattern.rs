use implicit_clone::sync::{IArray, IMap};
use implicit_clone::ImplicitClone;
use std::collections::HashMap;
use std::fmt;

pub mod grid;

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum RgbError {
    #[error("invalid input")]
    InvalidInput(String),
    #[error("string to u32 conversion")]
    Conversion(String),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn as_hex(self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b).to_lowercase()
    }

    pub fn from_hex(hex: String) -> Result<Rgb, RgbError> {
        let hex = hex.trim_start_matches('#');

        if hex.len() != 6 {
            return Err(RgbError::InvalidInput(hex.to_string()));
        }

        let int_value =
            u32::from_str_radix(hex, 16).map_err(|s| RgbError::Conversion(s.to_string()))?;

        let r = ((int_value >> 16) & 0xFF) as u8;
        let g = ((int_value >> 8) & 0xFF) as u8;
        let b = (int_value & 0xFF) as u8;

        Ok(Rgb { r, g, b })
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, ImplicitClone)]
pub struct Thread {
    pub color: Rgb,
    pub thread_type: ThreadType,
}

impl Thread {
    pub fn new(color: Rgb, thread_type: ThreadType) -> Self {
        Thread { color, thread_type }
    }
}

impl fmt::Display for Thread {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "color: {}", self.color)?;
        writeln!(f, "thread_type: {}", self.thread_type)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThreadType {
    SlashBackwards,
    SlashForwards,
    BorderTop,
    BorderRight,
    BorderBottom,
    BorderLeft,
}

impl ThreadType {
    pub fn as_str(&self) -> String {
        match &self {
            ThreadType::SlashBackwards => "SlashBackwards".to_string(),
            ThreadType::SlashForwards => "SlashForwards".to_string(),
            ThreadType::BorderTop => "BorderTop".to_string(),
            ThreadType::BorderRight => "BorderRight".to_string(),
            ThreadType::BorderBottom => "BorderBottom".to_string(),
            ThreadType::BorderLeft => "BorderLeft".to_string(),
        }
    }
}

impl fmt::Display for ThreadType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.as_str())
    }
}

// (column, row)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ImplicitClone)]
pub struct GridSize(pub usize, pub usize);

// (column, row)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ImplicitClone)]
pub struct GridCell(pub usize, pub usize);

#[derive(PartialEq)]
pub struct CellClick {
    pub thread_cell: (GridCell, IArray<Thread>),
    pub is_shift_active: bool,
}

pub type GridType = IMap<GridCell, IArray<Thread>>;
pub type RawGridType = HashMap<GridCell, IArray<Thread>>;
