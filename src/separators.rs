use crate::prelude::{AM_PM_SEP, DATE_SEP, DATE_TIME_SEP, TIME_SEP};

#[derive(Clone, Debug)]
pub struct Separators<'a> {
    pub date_sep:      &'a str,
    pub date_time_sep: &'a str,
    pub time_sep:      &'a str,
    pub am_pm_sep:     &'a str,
}

impl Separators<'_> {
    pub fn new<'a>(date_sep: &'a str, date_time_sep: &'a str, time_sep: &'a str, am_pm_sep: &'a str) -> Separators<'a> {
        Separators {
            date_sep,
            date_time_sep,
            time_sep,
            am_pm_sep,
        }
    }
}

impl Default for Separators<'_> {
    fn default() -> Self {
        Separators {
            date_sep:      DATE_SEP,
            date_time_sep: DATE_TIME_SEP,
            time_sep:      TIME_SEP,
            am_pm_sep:     AM_PM_SEP,
        }
    }
}

#[cfg(test)]
mod sep_tests {
    use super::*;

    #[test]
    fn test_separators() {
        let seps = Separators::default();
        assert_eq!(seps.date_sep, "_");
        assert_eq!(seps.date_time_sep, " ");
        assert_eq!(seps.time_sep, ".");
        match seps.am_pm_sep {
            "" | " " => {}
            _ => panic!("am_pm_sep should be either empty or a space"),
        }
    }
}
