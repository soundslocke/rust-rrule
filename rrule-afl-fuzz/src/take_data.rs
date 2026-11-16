#![allow(clippy::module_name_repetitions)]

use jiff::Timestamp;
use rrule::{NWeekday, TimeZone, Weekday, ZonedDateTime};

/// Takes the next `N` bytes, or none when fewer than `N` are left.
fn take_array<const N: usize>(input: &mut &[u8]) -> Option<[u8; N]> {
    let (bytes, rest) = input.split_first_chunk::<N>()?;
    *input = rest;
    Some(*bytes)
}

/// Uses max 1 + 5*4 = 21 bytes
pub fn take_vec_of_nweekday(input: &mut &[u8]) -> Vec<NWeekday> {
    let max_amount_of_items = 5;
    let amount_of_items = take_byte(input) % (max_amount_of_items + 1);
    (0..amount_of_items).map(|_| take_nweekday(input)).collect()
}

/// Uses max 1 + 5*8 = 41 bytes
pub fn take_vec_usize(input: &mut &[u8]) -> Vec<usize> {
    let max_amount_of_items = 5;
    let amount_of_items = take_byte(input) % (max_amount_of_items + 1);
    (0..amount_of_items)
        .map(|_| take_data_usize(input))
        .collect()
}

/// Uses max 1 + 5*8 = 41 bytes
pub fn take_vec_isize(input: &mut &[u8]) -> Vec<isize> {
    let max_amount_of_items = 5;
    let amount_of_items = take_byte(input) % (max_amount_of_items + 1);
    (0..amount_of_items)
        .map(|_| take_data_isize(input))
        .collect()
}

/// Uses max 1 + 5*1 = 6 bytes
pub fn take_vec_u8(input: &mut &[u8]) -> Vec<u8> {
    let max_amount_of_items = 5;
    let amount_of_items = take_byte(input) % (max_amount_of_items + 1);
    (0..amount_of_items).map(|_| take_data_u8(input)).collect()
}

/// Uses max 1 + 5*1 = 6 bytes
pub fn take_vec_i8(input: &mut &[u8]) -> Vec<i8> {
    let max_amount_of_items = 5;
    let amount_of_items = take_byte(input) % (max_amount_of_items + 1);
    (0..amount_of_items).map(|_| take_data_i8(input)).collect()
}

/// Uses max 1 + 5*2 = 11 bytes
pub fn take_vec_i16(input: &mut &[u8]) -> Vec<i16> {
    let max_amount_of_items = 5;
    let amount_of_items = take_byte(input) % (max_amount_of_items + 1);
    (0..amount_of_items).map(|_| take_data_i16(input)).collect()
}

/// Uses max 1 + 5*4 = 21 bytes
pub fn take_vec_i32(input: &mut &[u8]) -> Vec<i32> {
    let max_amount_of_items = 5;
    let amount_of_items = take_byte(input) % (max_amount_of_items + 1);
    (0..amount_of_items).map(|_| take_data_i32(input)).collect()
}

/// Uses 8+4+8 = 20 bytes
pub fn take_datetime(input: &mut &[u8]) -> ZonedDateTime {
    // Fold arbitrary input into jiff's supported instant range.
    let (min, max) = (Timestamp::MIN.as_second(), Timestamp::MAX.as_second());
    let seconds = min + take_data_i64(input).rem_euclid(max - min);
    // Below 10^9, so it always fits.
    let nanos = i32::try_from(take_data_u32(input) % 1_000_000_000).unwrap_or(0);
    Timestamp::new(seconds, nanos)
        .unwrap_or(Timestamp::UNIX_EPOCH)
        .to_zoned_datetime(TimeZone::UTC)
}

/// Uses 1 byte
/// If no bytes left it will always return default (`Mon`)
pub fn take_weekday(input: &mut &[u8]) -> Weekday {
    match take_byte(input) % 7 {
        0 => Weekday::Monday,
        1 => Weekday::Tuesday,
        2 => Weekday::Wednesday,
        3 => Weekday::Thursday,
        4 => Weekday::Friday,
        5 => Weekday::Saturday,
        _ => Weekday::Sunday,
    }
}

/// Uses 1 byte
/// If no bytes left it will always return default (`0`)
pub fn take_byte(input: &mut &[u8]) -> u8 {
    take_array(input).map_or(0, u8::from_be_bytes)
}

/// Uses max 1+2+1 = 4 byte
/// If no bytes left it will always return default (`Every(Mon)`)
pub fn take_nweekday(input: &mut &[u8]) -> NWeekday {
    match take_byte(input) % 2 {
        0 => NWeekday::Every(take_weekday(input)),
        _ => NWeekday::Nth(take_data_i16(input), take_weekday(input)),
    }
}

/// Uses 1 byte
/// If no bytes left it will always return default (`0`)
pub fn take_data_u8(input: &mut &[u8]) -> u8 {
    take_byte(input)
}

/// Uses 1 byte
/// If no bytes left it will always return default (`0`)
pub fn take_data_i8(input: &mut &[u8]) -> i8 {
    take_array(input).map_or(0, i8::from_be_bytes)
}

/// Uses 2 bytes
/// If no bytes left it will always return default (`0`)
pub fn take_data_i16(input: &mut &[u8]) -> i16 {
    take_array(input).map_or(0, i16::from_be_bytes)
}

/// Uses 4 bytes
/// If no bytes left it will always return default (`0`)
pub fn take_data_i32(input: &mut &[u8]) -> i32 {
    take_array(input).map_or(0, i32::from_be_bytes)
}

/// Uses 8 bytes
/// If no bytes left it will always return default (`0`)
pub fn take_data_i64(input: &mut &[u8]) -> i64 {
    take_array(input).map_or(0, i64::from_be_bytes)
}

/// Uses 2 bytes
/// If no bytes left it will always return default (`0`)
pub fn take_data_u16(input: &mut &[u8]) -> u16 {
    take_array(input).map_or(0, u16::from_be_bytes)
}

/// Uses 4 bytes
/// If no bytes left it will always return default (`0`)
pub fn take_data_u32(input: &mut &[u8]) -> u32 {
    take_array(input).map_or(0, u32::from_be_bytes)
}

/// Uses 8 bytes
/// If no bytes left it will always return default (`0`)
pub fn take_data_u64(input: &mut &[u8]) -> u64 {
    take_array(input).map_or(0, u64::from_be_bytes)
}

/// Uses max 8 bytes
/// If no bytes left it will always return default (`0`)
pub fn take_data_usize(input: &mut &[u8]) -> usize {
    take_array(input).map_or(0, usize::from_be_bytes)
}

/// Uses max 8 bytes
/// If no bytes left it will always return default (`0`)
pub fn take_data_isize(input: &mut &[u8]) -> isize {
    take_array(input).map_or(0, isize::from_be_bytes)
}

/// Uses 1 byte + [`take_datetime`]: a DTSTART in one of a few zones, chosen to reach
/// DST gaps and folds (including a 30-minute one) as well as UTC and floating time.
///
/// # Panics
///
/// If the time zone database lacks one of those zones, as on a host without tzdata.
pub fn take_zoned_start(input: &mut &[u8]) -> ZonedDateTime {
    let zone = match take_byte(input) % 7 {
        0 => TimeZone::UTC,
        1 => TimeZone::unknown(),
        2 => named_zone("America/Los_Angeles"),
        3 => named_zone("America/New_York"),
        4 => named_zone("Europe/London"),
        5 => named_zone("Australia/Lord_Howe"),
        _ => named_zone("Asia/Kolkata"),
    };
    take_datetime(input).with_time_zone(zone)
}

fn named_zone(name: &str) -> TimeZone {
    TimeZone::get(name).expect("the time zone database has every fuzzed zone")
}
