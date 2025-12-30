//! Date picker type definitions

use std::fmt;

/// Day of week
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Weekday {
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
}

impl Weekday {
    pub fn all_starting_from(first: Weekday) -> [Weekday; 7] {
        let all = [
            Weekday::Sun,
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
        ];
        let start = all.iter().position(|d| *d == first).unwrap_or(0);
        let mut ordered = [Weekday::Sun; 7];
        for i in 0..7 {
            ordered[i] = all[(start + i) % 7];
        }
        ordered
    }

    pub fn short_name(self) -> &'static str {
        match self {
            Weekday::Sun => "S",
            Weekday::Mon => "M",
            Weekday::Tue => "T",
            Weekday::Wed => "W",
            Weekday::Thu => "T",
            Weekday::Fri => "F",
            Weekday::Sat => "S",
        }
    }

    pub fn full_name(self) -> &'static str {
        match self {
            Weekday::Sun => "Sunday",
            Weekday::Mon => "Monday",
            Weekday::Tue => "Tuesday",
            Weekday::Wed => "Wednesday",
            Weekday::Thu => "Thursday",
            Weekday::Fri => "Friday",
            Weekday::Sat => "Saturday",
        }
    }
}

/// A calendar date
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl Date {
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    pub fn is_valid(self) -> bool {
        if !(1..=12).contains(&self.month) {
            return false;
        }
        let dim = days_in_month(self.year, self.month);
        self.day >= 1 && (self.day as u32) <= dim
    }

    pub fn today() -> Self {
        // For now, return a placeholder. In real app, use chrono or time crate
        Self::new(2025, 1, 1)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

/// A month in a specific year
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Month {
    pub year: i32,
    pub month: u8,
}

impl Month {
    pub fn new(year: i32, month: u8) -> Self {
        Self { year, month }
    }

    pub fn current() -> Self {
        // Placeholder - use real date library in production
        Self::new(2025, 1)
    }

    pub fn add_months(self, delta: i32) -> Self {
        let mut year = self.year;
        let mut month = self.month as i32;
        
        month += delta;
        year += (month - 1).div_euclid(12);
        month = ((month - 1).rem_euclid(12)) + 1;
        
        Self::new(year, month as u8)
    }

    pub fn first_day(self) -> Date {
        Date::new(self.year, self.month, 1)
    }

    pub fn last_day(self) -> Date {
        let days = days_in_month(self.year, self.month);
        Date::new(self.year, self.month, days as u8)
    }

    pub fn display_name(self) -> String {
        format!("{} {}", month_name(self.month), self.year)
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", month_name(self.month), self.year)
    }
}

/// Date selection (single date or range)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateSelection {
    Single(Date),
    Range { start: Date, end: Option<Date> },
}

impl DateSelection {
    pub fn contains(&self, date: Date) -> bool {
        match self {
            DateSelection::Single(d) => *d == date,
            DateSelection::Range { start, end } => {
                if let Some(end) = end {
                    date >= *start && date <= *end
                } else {
                    date == *start
                }
            }
        }
    }

    pub fn is_start(&self, date: Date) -> bool {
        match self {
            DateSelection::Single(d) => *d == date,
            DateSelection::Range { start, .. } => *start == date,
        }
    }

    pub fn is_end(&self, date: Date) -> bool {
        match self {
            DateSelection::Single(d) => *d == date,
            DateSelection::Range { end: Some(end), .. } => *end == date,
            DateSelection::Range { end: None, .. } => false,
        }
    }

    pub fn is_in_range(&self, date: Date) -> bool {
        match self {
            DateSelection::Single(_) => false,
            DateSelection::Range { start, end: Some(end) } => {
                date > *start && date < *end
            }
            DateSelection::Range { .. } => false,
        }
    }
}

// ============================================================================
// Utility functions
// ============================================================================

pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

pub fn days_in_month(year: i32, month: u8) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

pub fn month_name(month: u8) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    }
}

pub fn month_short_name(month: u8) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "???",
    }
}

/// Calculate weekday for a date using Sakamoto's algorithm
pub fn weekday_for_date(date: Date) -> Weekday {
    let mut y = date.year;
    let m = date.month as i32;
    let d = date.day as i32;
    static T: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    
    if m < 3 {
        y -= 1;
    }
    
    let w = (y + y / 4 - y / 100 + y / 400 + T[(m - 1) as usize] + d) % 7;
    
    match w {
        0 => Weekday::Sun,
        1 => Weekday::Mon,
        2 => Weekday::Tue,
        3 => Weekday::Wed,
        4 => Weekday::Thu,
        5 => Weekday::Fri,
        _ => Weekday::Sat,
    }
}

pub fn weekday_index(w: Weekday) -> i32 {
    match w {
        Weekday::Sun => 0,
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
    }
}
