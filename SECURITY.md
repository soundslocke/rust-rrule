## Security

The crate aims to be safe to use with rules from untrusted users or public
APIs, but that cannot be guaranteed. These are the main concerns.

### Denial of Service (DoS) by panic

Input that triggers a panic could crash the application or server. Parsing and
iteration return an error or stop rather than panic where they can, and an AFL
fuzzer (`rrule-afl-fuzz`) exercises both. As a further guard, wrap calls in
[`catch_unwind`](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html),
which does not work with [`panic = "abort"`](https://doc.rust-lang.org/cargo/reference/profiles.html#panic).
Do not reuse an `RRule`, `RRuleSet` or iterator after it has panicked.

Arithmetic that a large `INTERVAL` could overflow is checked and returns an
error. Still, building with overflow checks off is strongly discouraged, as an
overflow elsewhere would then give wrong results instead of a panic.

### Denial of Service (DoS) by CPU exhaustion

A rule can describe an occurrence that never exists, so a search for it never
ends. The [iteration limit](README.md#validation_limits) stops such a search,
by default after 100,000 steps, which can still take a few seconds. Lower it
with `RRuleSet::iteration_limit` for rules you do not control.
`RRuleSet::all_unchecked` and direct iteration have no iteration limit unless
`iteration_limit` or `RRuleSet::limit` turns it on. For a hard time bound, expand rules in a separate process
that can be killed; a Rust thread cannot be stopped from outside.

### Denial of Service (DoS) by memory exhaustion

A rule can recur forever, or for millions of occurrences before year 9999.
`RRuleSet::all(limit)` caps how many occurrences it collects. `all_unchecked`
and `collect` on the iterator do not, so use them only on trusted rules, or
call `take(n)` on the iterator.
