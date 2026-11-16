pub mod take_data;
pub mod take_rrule;

use take_rrule::take_rrule_from_data;

use afl::fuzz;
use core::str::FromStr;
use rrule::RRuleSet;
use std::env::var;
use std::str::from_utf8;

fn main() {
    // RRULE_FUZZ_TARGET picks the target below; the default stays the structured one.
    let fuzz_selector: u8 = var("RRULE_FUZZ_TARGET")
        .ok()
        .and_then(|t| t.parse().ok())
        .unwrap_or(2);
    match fuzz_selector {
        // Fuzz the RRuleSet from string
        0 => {
            fuzz!(|data: &[u8]| {
                if let Ok(s) = from_utf8(data) {
                    // Any result is acceptable; only a panic is a finding.
                    let _ = RRuleSet::from_str(s);
                }
            });
        }
        // Fuzz the RRuleSet from string, then expand it
        1 => {
            fuzz!(|data: &[u8]| {
                if let Ok(s) = from_utf8(data) {
                    // Any result is acceptable; only a panic is a finding.
                    if let Ok(rule) = RRuleSet::from_str(s) {
                        let _ = rule.all(50);
                    }
                }
            });
        }
        // Fuzz directly to RRule, skip parser
        2 => {
            fuzz!(|data: &[u8]| {
                // Too little data to build an `RRule` is not a finding.
                if let Some(rule) = take_rrule_from_data(data) {
                    let _ = rule.all(50);
                }
            });
        }
        _ => {}
    }
}
