use std::fmt::{self, Display};
use std::str::FromStr;

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

impl FromStr for Stamp {
    type Err = eyre::ErrReport;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split_whitespace();

        // Parse date part
        let date_str = parts
            .next()
            .ok_or("Missing date part")
            .expect("Missing date part");
        let date = DateParts::from_str(date_str)?;

        // Parse time and AM/PM parts, if present
        let military_time_part = parts.next();
        let time_part = parts.next();
        let am_pm_part = parts.next();

        let time = match (military_time_part, time_part, am_pm_part) {
            (Some(military_str), Some(time_str), Some(am_pm_str)) => {
                Some(TimeStampParts::from_parts(military_str, time_str, am_pm_str)?)
            }
            _ => None,
        };

        Ok(Stamp { date, time })
    }
}

impl Stamp {
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

impl FromStr for DateParts {
    type Err = eyre::ErrReport;

    fn from_str(s: &str) -> Result<Self> {
        let potential_seps = ['-', '/', '.'];

        // let date_components: Vec<&str> = s.split('/').collect();
        let date_components: Vec<&str> = if potential_seps.iter().any(|&sep| s.contains(sep)) {
            let sep = potential_seps.iter().find(|&&sep| s.contains(sep)).unwrap();
            s.split(*sep).collect()
        } else {
            return Err(eyre!("Invalid date format: no valid separator found"));
        };

        if date_components.len() != 3 {
            return Err(eyre!("Invalid date format"));
        }

        Ok(DateParts {
            day:   date_components[0].to_string(),
            month: date_components[1].to_string(),
            year:  date_components[2].to_string(),
        })
    }
}

impl DateParts {
    fn formatted(&self, date_sep: &str) -> String {
        let date_parts = vec![self.year.as_str(), self.month.as_str(), self.day.as_str()];
        let mut buf = String::new();
        for part in date_parts {
            buf.push_str(part);
            buf.push_str(date_sep);
        }
        // Remove the trailing separator
        // Safe, as we did it outselves in the same fn
        buf.pop();
        buf
    }
}

// Struct to represent time parts along with AM or PM
pub struct TimeStampParts {
    time:     String,
    am_or_pm: AmOrPm,
}

impl TimeStampParts {
    fn from_parts<S: AsRef<str>>(military_str: S, time_part: S, am_pm_str: S) -> Result<Self> {
        let am_or_pm = AmOrPm::from_str(am_pm_str.as_ref()).unwrap_or(AmOrPm::Am);
        let time = format!("{} {}", military_str.as_ref(), time_part.as_ref());
        Ok(TimeStampParts { time, am_or_pm })
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

impl FromStr for AmOrPm {
    type Err = eyre::ErrReport;

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
        let seps = crate::Seperators::default();
        let new_date = flip_date_format(&creation_time, &seps).unwrap();

        assert!(new_date.contains("_"));
    }

    #[should_panic]
    #[test]
    fn test_stamp_converter_no_time() {
        let creation_time = "26/05/2022".to_string();
        let seps = crate::Seperators::default();
        let new_date = flip_date_format(&creation_time, &seps).unwrap();

        assert_eq!(new_date, "2022-05-26");
    }

    #[test]
    fn test_date_parts_formatted() {
        let date = DateParts {
            day:   "26".to_string(),
            month: "05".to_string(),
            year:  "2022".to_string(),
        };
        let formatted_date = date.formatted("/");
        assert_eq!(formatted_date, "2022/05/26");
    }
}
