use crate::print_all_datetimes;
use rrule::RRuleSet;
use std::panic::catch_unwind;
use std::str::FromStr;
use std::str::from_utf8;

pub fn from_crash_file(id: u32, data: &[u8]) {
    println!("--------- Test file {id} -----------");

    let data_string = from_utf8(data).unwrap();
    println!("Test {id} String: \n\n{data_string:?}\n");
    let result = catch_unwind(|| {
        parse_rrule_from_string(data_string);
    });
    println!("Test {id} status: {result:?}");
    println!("--------- Done test {id} -----------");
}

pub fn parse_rrule_from_string(rrule: &str) {
    match RRuleSet::from_str(rrule) {
        Ok(rrule) => {
            println!("RRule data: {rrule:#?}");
            let result = rrule.all(50);
            print_all_datetimes(&result.dates);
            if result.limited {
                println!("RRule was limited");
            }
        }
        Err(err) => println!("RRule could not be parsed: {err}"),
    };
}
