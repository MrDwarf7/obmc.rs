use std::fmt::{self, Display};

use eyre::eyre;

pub use crate::prelude::*;

// Function to flip the date format using the Stamp struct
pub fn flip_date_format(to_section: &str, seps: &crate::Seperators) -> Result<String> {
    Ok(Stamp::from_str(to_section)?.formatted(seps))
}

// Main struct to represent the complete timestamp
pub struct Stamp {
    date: DateParts,
    time: Option<TimeStampParts>,
}

impl Stamp {
    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split_whitespace();

        // Parse date part
        let date_str = parts
            .next()
            .ok_or("Missing date part")
            .expect("Missing date part");
        let date = DateParts::from_str(date_str)?;

        // Parse time and AM/PM parts, if present
        let time_part = parts.next();
        let am_pm_part = parts.next();

        let time = match (time_part, am_pm_part) {
            (Some(time_str), Some(am_pm_str)) => Some(TimeStampParts::from_parts(time_str, am_pm_str)?),
            _ => None,
        };

        Ok(Stamp { date, time })
    }

    fn formatted(&self, seps: &crate::Seperators) -> String {
        let date_formatted = self.date.formatted(seps.date_sep);
        if let Some(ref time_stamp) = self.time {
            format!("{}{}{}", date_formatted, &seps.date_time_sep, time_stamp.formatted(seps))
        } else {
            date_formatted
        }
    }
}

// Struct to represent date parts
pub struct DateParts {
    day:   String,
    month: String,
    year:  String,
}

impl DateParts {
    fn from_str(s: &str) -> Result<Self> {
        let date_components: Vec<&str> = s.split('/').collect();
        if date_components.len() != 3 {
            return Err(eyre!("Invalid date format"));
        }

        Ok(DateParts {
            day:   date_components[0].to_string(),
            month: date_components[1].to_string(),
            year:  date_components[2].to_string(),
        })
    }

    fn formatted(&self, date_sep: &str) -> String {
        let date_parts = vec![&self.year, &self.month, &self.day];
        let mut buf = String::new();
        for part in date_parts {
            buf.push_str(part);
            buf.push_str(date_sep);
        }
        buf.pop(); // Remove the trailing separator
        buf
    }
}

// Struct to represent time parts along with AM or PM
pub struct TimeStampParts {
    time:     String,
    am_or_pm: AmOrPm,
}

impl TimeStampParts {
    fn from_parts(time_part: &str, am_pm_str: &str) -> Result<Self> {
        let am_or_pm = AmOrPm::from_str(am_pm_str)?;
        Ok(TimeStampParts {
            time: time_part.to_string(),
            am_or_pm,
        })
    }

    fn formatted(&self, seps: &crate::Seperators) -> String {
        // Replace ':' with '-'
        let time_formatted = self.time.replace(":", seps.time_sep);
        format!("{}{}{}", time_formatted, &seps.am_pm_sep, self.am_or_pm)
    }
}

// Enum to represent AM or PM
pub enum AmOrPm {
    Am,
    Pm,
}

impl AmOrPm {
    fn from_str(s: &str) -> Result<Self> {
        match s.to_uppercase().as_str() {
            "AM" => Ok(AmOrPm::Am),
            "PM" => Ok(AmOrPm::Pm),
            _ => Err(eyre!("Invalid value for AmOrPm")),
        }
    }
}

impl Display for AmOrPm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AmOrPm::Am => write!(f, "AM"),
            AmOrPm::Pm => write!(f, "PM"),
        }
    }
}

#[cfg(test)]
mod stamp_tests {
    use super::*;

    #[test]
    fn test_stamp_converter() {
        let creation_time = "26/05/2022 12:40 AM".to_string();
        let seps = crate::Seperators::new(DATE_SEP, DATE_TIME_SEP, TIME_SEP, AM_PM_SEP);
        let new_date = flip_date_format(&creation_time, &seps).unwrap();

        assert!(new_date.contains("_"));
    }
}
