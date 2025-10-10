use crate::prelude::{AM_PM_SEP, DATE_SEP, DATE_TIME_SEP, TIME_SEP};

#[derive(Clone)]
pub struct Seperators<'a> {
    pub date_sep:      &'a str,
    pub date_time_sep: &'a str,
    pub time_sep:      &'a str,
    pub am_pm_sep:     &'a str,
}

impl Seperators<'_> {
    pub fn new<'a, S>(
        date_sep: &'a S,
        date_time_sep: &'a S,
        time_sep: &'a S,
        am_pm_sep: &'a S,
    ) -> Seperators<'a>
    where
        S: AsRef<str> + ToString + ?Sized,
    {
        Seperators {
            date_sep:      date_sep.as_ref(),
            time_sep:      time_sep.as_ref(),
            date_time_sep: date_time_sep.as_ref(),
            am_pm_sep:     am_pm_sep.as_ref(),
        }
    }
}

impl Default for Seperators<'_> {
    fn default() -> Self {
        Seperators {
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
    fn test_seperators() {
        let seps = Seperators::default();
        assert_eq!(seps.date_sep, "_");
        assert_eq!(seps.date_time_sep, " ");
        assert_eq!(seps.time_sep, ".");
        match seps.am_pm_sep {
            "" | " " => {}
            _ => panic!("am_pm_sep should be either empty or a space"),
        }
    }
}
