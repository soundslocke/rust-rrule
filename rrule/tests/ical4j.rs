//! Runs ical4j's Recur expansion tests (tests/data/ical4j_recur_tests.txt, BSD-3-Clause)
//! against this crate.
mod common;

use common::corpus::run;
use std::env::var;

#[test]
fn ical4j_corpus() {
    let path = var("ICAL4J_CORPUS").unwrap_or_else(|_| {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/data/ical4j_recur_tests.txt"
        )
        .into()
    });
    run("ical4j", &path);
}
