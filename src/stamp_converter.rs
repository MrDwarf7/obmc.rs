use std::fmt::{self, Display};
use std::str::FromStr;

use eyre::eyre;

use crate::Separators;
use crate::prelude::*;

/// Flip `DD/MM/YYYY ...` (and optional dual 24h + 12h time) into sort-friendly
/// `YYYY_MM_DD 24h 12hAM` form using the given separators.
pub fn flip_date_format(to_section: &str, seps: &Separators<'_>) -> Result<String> {
    Ok(Stamp::from_str(to_section)?.formatted(seps))
}

pub struct Stamp {
    date: DateParts,
    time: Option<TimeStampParts>,
}

impl FromStr for Stamp {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split_whitespace();

        let date_str = parts
            .next()
            .ok_or_else(|| eyre!("Missing date part in stamp: {s:?}"))?;
        let date = DateParts::from_str(date_str)?;

        // Dual-time layout from get_creation_time:
        //   DD/MM/YYYY HH:MM II:MM AM|PM
        // (24h for sorting, 12h for reading)
        let military_time_part = parts.next();
        let time_part = parts.next();
        let am_pm_part = parts.next();

        let time = match (military_time_part, time_part, am_pm_part) {
            (Some(military_str), Some(time_str), Some(am_pm_str)) => {
                Some(TimeStampParts::from_parts(military_str, time_str, am_pm_str)?)
            }
            // Date-only input is valid (no time suffix in output).
            (None, None, None) => None,
            // Legacy single 12h form: "DD/MM/YYYY HH:MM AM"
            (Some(time_str), Some(am_pm_str), None) => {
                Some(TimeStampParts::from_single(time_str, am_pm_str)?)
            }
            _ => {
                return Err(eyre!("Invalid time portion in stamp: {s:?}"));
            }
        };

        Ok(Stamp { date, time })
    }
}

impl Stamp {
    fn formatted(&self, seps: &Separators<'_>) -> String {
        let date_formatted = self.date.formatted(seps.date_sep);
        if let Some(ref time_stamp) = self.time {
            format!("{}{}{}", date_formatted, seps.date_time_sep, time_stamp.formatted(seps))
        } else {
            date_formatted
        }
    }
}

pub struct DateParts {
    day:   String,
    month: String,
    year:  String,
}

impl FromStr for DateParts {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        let potential_seps = ['-', '/', '.'];

        let date_components: Vec<&str> =
            if let Some(sep) = potential_seps.iter().find(|&&sep| s.contains(sep)) {
                s.split(*sep).collect()
            } else {
                return Err(eyre!("Invalid date format: no valid separator found in {s:?}"));
            };

        if date_components.len() != 3 {
            return Err(eyre!("Invalid date format: expected 3 components, got {date_components:?}"));
        }

        for (i, c) in date_components.iter().enumerate() {
            if c.is_empty() || !c.chars().all(|ch| ch.is_ascii_digit()) {
                return Err(eyre!("Invalid date format: component {i} is not numeric: {c:?} in {s:?}"));
            }
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
        // YYYY_MM_DD (trailing sep popped)
        let mut buf = String::with_capacity(10 + date_sep.len() * 2);
        buf.push_str(&self.year);
        buf.push_str(date_sep);
        buf.push_str(&self.month);
        buf.push_str(date_sep);
        buf.push_str(&self.day);
        buf
    }
}

/// Holds both military (24h) and 12h wall time for the dual-time filename layout.
pub struct TimeStampParts {
    /// e.g. "14:40 02:40" or just "12:40" for legacy single-time input
    time:     String,
    am_or_pm: AmOrPm,
}

impl TimeStampParts {
    fn from_parts(military_str: &str, time_part: &str, am_pm_str: &str) -> Result<Self> {
        let am_or_pm = AmOrPm::from_str(am_pm_str)?;
        let time = format!("{military_str} {time_part}");
        Ok(TimeStampParts { time, am_or_pm })
    }

    fn from_single(time_str: &str, am_pm_str: &str) -> Result<Self> {
        let am_or_pm = AmOrPm::from_str(am_pm_str)?;
        Ok(TimeStampParts {
            time: time_str.to_string(),
            am_or_pm,
        })
    }

    fn formatted(&self, seps: &Separators<'_>) -> String {
        let time_formatted = self.time.replace(':', seps.time_sep);
        format!("{time_formatted}{}{}", seps.am_pm_sep, self.am_or_pm)
    }
}

pub enum AmOrPm {
    Am,
    Pm,
}

impl FromStr for AmOrPm {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_uppercase().as_str() {
            "AM" => Ok(AmOrPm::Am),
            "PM" => Ok(AmOrPm::Pm),
            _ => Err(eyre!("Invalid value for AmOrPm: {s:?}")),
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
    fn test_stamp_converter_dual_time() {
        // Matches get_creation_time output: 24h + 12h + AM/PM
        let creation_time = "26/05/2022 14:40 02:40 PM";
        let seps = Separators::default();
        let new_date = flip_date_format(creation_time, &seps).unwrap();
        assert_eq!(new_date, "2022_05_26 14.40 02.40PM");
    }

    #[test]
    fn test_stamp_converter_legacy_single_time() {
        let creation_time = "26/05/2022 12:40 AM";
        let seps = Separators::default();
        let new_date = flip_date_format(creation_time, &seps).unwrap();
        assert_eq!(new_date, "2022_05_26 12.40AM");
    }

    #[test]
    fn test_stamp_converter_date_only() {
        let creation_time = "26/05/2022";
        let seps = Separators::default();
        let new_date = flip_date_format(creation_time, &seps).unwrap();
        assert_eq!(new_date, "2022_05_26");
    }

    #[test]
    fn test_date_parts_formatted() {
        let date = DateParts {
            day:   "26".to_string(),
            month: "05".to_string(),
            year:  "2022".to_string(),
        };
        assert_eq!(date.formatted("/"), "2022/05/26");
        assert_eq!(date.formatted("_"), "2022_05_26");
    }
}
