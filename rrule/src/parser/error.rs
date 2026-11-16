#![allow(clippy::module_name_repetitions)]
use thiserror::Error;

/// Why a recurrence string could not be parsed.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// A `TZID` names a zone that is not in the time zone database.
    #[error("`{0}` is not a valid timezone.")]
    InvalidTimezone(String),
    /// A date or date-time is well formed but does not exist, such as February 30, or is
    /// outside the range of supported instants.
    #[error("`{value}` is not a valid datetime format for `{property}`.")]
    InvalidDateTime {
        /// The value as written.
        value: String,
        /// The property it was given for, such as `DTSTART` or `UNTIL`.
        property: String,
    },
    /// A date or date-time is not in the `YYYYMMDD[THHMMSS[Z]]` form.
    #[error("`{0}` is not a valid datetime format.")]
    InvalidDateTimeFormat(String),
    /// `FREQ` is not one of the RFC 5545 frequencies.
    #[error("`{0}` is not a valid frequency.")]
    InvalidFrequency(String),
    /// A `BYDAY` entry is not a weekday, optionally prefixed by an ordinal.
    #[error("`{0}` is not a valid weekday. Valid values are `MO`, `TU`, `WE`, `TH`, `FR`, `SA` and `SU`.")]
    InvalidWeekday(String),
    /// `WKST` is not a weekday.
    #[error("`{0}` is not a valid weekday start. Valid values are `MO`, `TU`, `WE`, `TH`, `FR`, `SA` and `SU`.")]
    InvalidWeekdayStart(String),
    /// `BYEASTER` is not an integer that fits in an `i16`.
    #[error("`{0}` is not a valid BYEASTER value.")]
    InvalidByEaster(String),
    /// `INTERVAL` is not a non-negative integer that fits in a `u16`.
    #[error("`{0}` is not a valid INTERVAL value.")]
    InvalidInterval(String),
    /// `COUNT` is not a non-negative integer that fits in a `u32`.
    #[error("`{0}` is not a valid COUNT value.")]
    InvalidCount(String),
    /// `BYHOUR` is not a comma-separated list of integers.
    #[error("`{0}` is not a valid BYHOUR value. Expected a comma-separated list of values in range 0..=23, e.g. `1,3,4`")]
    InvalidByHour(String),
    /// `BYWEEKNO` is not a comma-separated list of integers.
    #[error("`{0}` is not a valid BYWEEKNO value. Expected a comma-separated list of values in range -53..=53, e.g. `-1,30,53`")]
    InvalidByWeekNo(String),
    /// `BYYEARDAY` is not a comma-separated list of integers.
    #[error("`{0}` is not a valid BYYEARDAY value. Expected a comma-separated list of values in range -366..=366, e.g. `-100,`")]
    InvalidByYearDay(String),
    /// `BYMONTHDAY` is not a comma-separated list of integers.
    #[error("`{0}` is not a valid BYMONTHDAY value. Expected a comma-separated list of values in range -31..=31, e.g. `-30,10`")]
    InvalidByMonthDay(String),
    /// `BYMONTH` is not a comma-separated list of integers.
    #[error("`{0}` is not a valid BYMONTH value. Expected a comma-separated list of values in range 1..=12, e.g. `6,9,10`")]
    InvalidByMonth(String),
    /// `BYMINUTE` is not a comma-separated list of integers.
    #[error("`{0}` is not a valid BYMINUTE value. Expected a comma-separated list of values in range 0..=59, e.g. `0,15,30,45`")]
    InvalidByMinute(String),
    /// `BYSECOND` is not a comma-separated list of integers.
    #[error("`{0}` is not a valid BYSECOND value. Expected a comma-separated list of values in range 0..=59, e.g. `0,15,30,45`")]
    InvalidBySecond(String),
    /// `BYSETPOS` is not a comma-separated list of integers.
    #[error("`{0}` is not a valid BYSETPOS value. Expected a comma-separated list of integers, e.g. `-3,1`")]
    InvalidBySetPos(String),
    /// A rule is missing a part it requires, such as `FREQ`.
    #[error("The property `{0}` wasn't found, and it is required.")]
    MissingProperty(String),
    /// A parameter or rule part is not written as `key=value`.
    #[error(
        "`{0}` is a malformed property parameter. Parameter should be specified as `key=value`"
    )]
    InvalidParameterFormat(String),
    /// A parameter or rule part name is not one this crate knows.
    #[error("`{0}` is not a valid property parameter.")]
    UnrecognizedParameter(String),
    /// A parameter or rule part appears more than once on one line.
    #[error("Found duplicate property for `{0}`, properties and parameters need to be unique.")]
    DuplicateProperty(String),
    /// The input has no `DTSTART`.
    #[error("Missing start date. There needs to be a unique start date which the iteration can start from.")]
    MissingStartDate,
    /// The input has neither an `RRULE` nor an `RDATE`.
    #[error("Missing date generation property. There needs to be at least one `RRULE` or `RDATE` to generate occurrences.")]
    MissingDateGenerationRules,
    /// An `RRULE` or `EXRULE` line has parameters, which those properties do not take.
    #[error("Property parameters aren't supported for RRULE / EXRULE, found parameters: `{0}`")]
    PropertyParametersNotSupported(String),
    /// A line starts with a property name other than `RRULE`, `EXRULE`, `DTSTART`,
    /// `RDATE` or `EXDATE`.
    #[error(
        "`{0}` is not a valid property name, expected one of: `RRULE,EXRULE,DTSTART,RDATE,EXDATE`"
    )]
    UnrecognizedPropertyName(String),
    /// A parameter disagrees with the value it describes, such as `VALUE=DATE` on a
    /// date-time.
    #[error("Property parameter `{parameter}` was set to have value `{parameter_value}`, but found `{found_value}` ")]
    ParameterValueMismatch {
        /// The parameter's name, such as `VALUE`.
        parameter: String,
        /// What the parameter says the value is.
        parameter_value: String,
        /// What the value actually is.
        found_value: String,
    },
}
