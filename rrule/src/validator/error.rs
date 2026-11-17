#![allow(clippy::module_name_repetitions)]

use thiserror::Error;

use crate::Frequency;

/// Why a parsed or built rule breaks RFC 5545 or this crate's limits.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// `BYSETPOS` is set without any other `BYxxx` rule part for it to select from.
    #[error("BYSETPOS should only be used in conjunction with another BYxxx rule part.")]
    BySetPosWithoutByRule,
    /// A rule part holds a value it may never take, such as `0` in `BYMONTHDAY`.
    #[error("`{field}` can't be `{value}`.")]
    InvalidFieldValue {
        /// The rule part, such as `BYMONTHDAY`.
        field: String,
        /// The forbidden value.
        value: String,
    },
    /// A value is outside its allowed range.
    #[error(
        "`{field}` is `{value}`, but is not allowed outside the range: `{start_idx}..={end_idx}`."
    )]
    InvalidFieldValueRange {
        /// The rule part, such as `BYHOUR`, or `DTSTART` or `YEAR` for a year out of range.
        field: String,
        /// The value that is out of range.
        value: String,
        /// The lowest allowed value.
        start_idx: String,
        /// The highest allowed value.
        end_idx: String,
    },
    /// A value is outside the range its rule part allows at the rule's frequency.
    #[error(
        "`{field}` is `{value}`, but with the current frequency ({freq}) is not allowed \
            outside the range: `{start_idx}..={end_idx}`."
    )]
    InvalidFieldValueRangeWithFreq {
        /// The rule part, such as `BYSETPOS` or `BYDAY`.
        field: String,
        /// The value that is out of range.
        value: String,
        /// The rule's frequency.
        freq: Frequency,
        /// The lowest allowed value.
        start_idx: String,
        /// The highest allowed value.
        end_idx: String,
    },
    /// A rule part is not allowed at the rule's frequency, such as `BYWEEKNO` outside
    /// `FREQ=YEARLY`.
    #[error("`{by_rule}` can not be used with the current frequency ({freq}).")]
    InvalidByRuleAndFrequency {
        /// The rule part, such as `BYWEEKNO`.
        by_rule: String,
        /// The rule's frequency.
        freq: Frequency,
    },
    /// `UNTIL` is earlier than `DTSTART`.
    #[error("`UNTIL` is `{until}`, but `DTSTART` (`{dt_start}`) is later. That should not be happening.")]
    UntilBeforeStart {
        /// `UNTIL`, as RFC 3339.
        until: String,
        /// `DTSTART`, as RFC 3339.
        dt_start: String,
    },
    /// The rule has no time parts to place occurrences with: `FREQ=HOURLY` with neither
    /// `BYMINUTE` nor `BYSECOND`, `FREQ=MINUTELY` without `BYSECOND`, or a coarser
    /// frequency with none of `BYHOUR`, `BYMINUTE` and `BYSECOND`.
    #[error(
        "Unable to generate a timeset for the RRULE. Please specify a BYHOUR, BYMINUTE or BYSECOND"
    )]
    UnableToGenerateTimeset,
    /// `BYEASTER` is used without all of `BYHOUR`, `BYMINUTE` and `BYSECOND`.
    #[cfg(feature = "by-easter")]
    #[error("`BYEASTER` can only be used when `BYHOUR`, `BYMINUTE` and `BYSECOND` are set.")]
    InvalidByRuleWithByEaster,
    /// `UNTIL` is in a zone RFC 5545 does not allow for `DTSTART`'s zone: it must be
    /// UTC when `DTSTART` has a zone, and floating or UTC when `DTSTART` is floating.
    #[error(
        "The value of `DTSTART` was specified in {dt_start_tz} timezone, but `UNTIL` was specified in timezone {until_tz}. Allowed timezones for `UNTIL` with the given start date timezone are: `{expected:?}`"
    )]
    DtStartUntilMismatchTimezone {
        /// `DTSTART`'s zone.
        dt_start_tz: String,
        /// `UNTIL`'s zone.
        until_tz: String,
        /// The zones `UNTIL` could have been in.
        expected: Vec<String>,
    },
}
