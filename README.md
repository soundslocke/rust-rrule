<h1 align="center">RRule.rs</h1>
<p align="center">A pure and efficient Rust implementation of recurrence rules as defined in the iCalendar RFC.</p>

## Specification

This crate follows the [iCalendar (RFC 5545) specification][ical_spec] for the "Recurrence Rule",
which corresponds to the `RRule` type. A rule may be preceded by a
["DTSTART" property][dtstart_property] on its own line.

A "BYEASTER" filter is available behind the `by-easter` feature flag.

### RRuleSet

`RRuleSet` combines `RRule`s with other properties:

- [RRULE](https://icalendar.org/iCalendar-RFC-5545/3-8-5-3-recurrence-rule.html):
  rules whose occurrences are included. (Union, `A ∪ B`)
- [RDATE](https://icalendar.org/iCalendar-RFC-5545/3-8-5-2-recurrence-date-times.html):
  date-times always included. (Union, `A ∪ B`)
- [EXRULE](https://datatracker.ietf.org/doc/html/rfc2445#section-4.8.5.2):
  rules whose occurrences are excluded. (Difference, `A \ B`)
- [EXDATE](https://icalendar.org/iCalendar-RFC-5545/3-8-5-1-exception-date-times.html):
  date-times always excluded. (Difference, `A \ B`)

EXRULE comes from RFC 2445, which [RFC 5545][ical_spec] obsoletes. It works
exactly like RRULE except that it excludes dates, and needs the `exrule` feature flag.

### Google Calendar

Where RFC 5545 is silent or Google Calendar departs from it, the crate follows
python-dateutil by default. `RRuleSet::google_compat()` expands as Google does
instead, turning on all of:

- `dtstart_always_occurs(true)`: a DTSTART the rule does not generate is still
  the first occurrence, outside COUNT, and an EXDATE can remove it.
- `yearly_bymonthday_uses_dtstart_month(true)`: a YEARLY rule with BYMONTHDAY
  but no BYMONTH keeps to DTSTART's month, instead of repeating in every month.
- `date_start_ignores_time_parts(true)`: with a DATE DTSTART (an all-day
  series), DAILY and coarser rules ignore BYHOUR, BYMINUTE and BYSECOND, which
  RFC 5545 forbids there, so every occurrence stays at midnight.

Google also stops a series at its 730th instance, counted from DTSTART, which
`google_compat` does not do; `all(730)` gives the same bound.
`rrule/tests/google.rs` checks expansion against instances recorded from Google Calendar.

## Library Usage

```rust
use rrule::RRuleSet;

// Starts 2012-02-01 and occurs daily for 3 days.
let rrule: RRuleSet = "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3".parse().unwrap();

// Take at most 100 occurrences, since a rule can recur forever.
let result = rrule.all(100);
assert_eq!(result.dates.len(), 3);
```

More examples are in [`rrule/examples`](rrule/examples).

## Command Line Tool Usage

```bash
cargo install --git https://github.com/soundslocke/rust-rrule rrule --features cli-tool
rrule "DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3"
```

## Limits

Dates are [Jiff][jiff] `ZonedDateTime` values; the `chrono` feature adds
conversions for callers still on chrono. A rule's DTSTART must be in years
0..=9999, the years RFC 5545's four-digit year can write. Iteration stops at the
end of 9999, or at Jiff's last instant (9999-12-30T22:00Z) where that comes first.

Time zones come from the IANA database as Jiff finds it: the system copy where
there is one, or a bundled copy on platforms without one. A `TZID` using a
legacy alias such as `US/Pacific` resolves only if the database in use still
carries that alias; many Linux distributions now ship tzdata without them.

<a name="validation_limits"></a>
The iteration limit is how many steps iteration takes without finding an
occurrence before it gives up and sets `RRuleResult::limited`. It defaults to
100,000 and applies to `RRuleSet::all` and to sets built with `RRuleSet::limit`.
A rule that can never match costs the full limit, up to a few seconds.
`RRuleSet::iteration_limit(n)` sets it per rule set, for example lower for a
service expanding rules it does not control; it then also applies to
`all_unchecked` and direct iteration, which are otherwise unbounded.

Read the [security notes](SECURITY.md) before expanding rules from untrusted input.

## Inspired by

- [python-dateutil](https://github.com/dateutil/dateutil)
- [rrule.js](https://github.com/jkbrzt/rrule)

## License

Licensed under the [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) license, and
contributions are licensed the same way.

[jiff]: https://github.com/BurntSushi/jiff
[ical_spec]: https://icalendar.org/iCalendar-RFC-5545/3-3-10-recurrence-rule.html
[dtstart_property]: https://icalendar.org/iCalendar-RFC-5545/3-8-2-4-date-time-start.html
