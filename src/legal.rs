//! Constants pulled from the files
//! `LICENSE`, `NOTICE` and `THANKS`.

const LICENSE: &str = include_str!("../LICENSE");

const THANKS: &str = include_str!("../THANKS");

/// 1-based line where stuff relevant to warranty starts
/// in the GPLv3.
const WARRANTY_START_LINE: usize = 589;

/// 1-based line where stuff relevant to warranty ends
/// in the GPLv3.
const WARRANTY_END_LINE: usize = 617;

/// Full text of the GPLv3.
pub fn license() -> &'static str {
    LICENSE
}

/// Attributions for external dependencies used to
/// build spectapir, e.g. pulseaudio.
pub fn thanks() -> &'static str {
    THANKS
}

/// Excerpt of the GPLv3 that is relevant for warranty
/// topics (lines 589--619).
pub fn warranty() -> &'static str {
    let mut line_feeds = LICENSE
        .as_bytes()
        .into_iter()
        .enumerate()
        .filter(|(_, b)| **b == 0x0a)
        .skip(WARRANTY_START_LINE);
    let warranty_start_byte = line_feeds.next().unwrap().0;
    let warranty_end_byte = line_feeds
        .skip(WARRANTY_END_LINE - WARRANTY_START_LINE)
        .next()
        .unwrap()
        .0;
    &LICENSE[warranty_start_byte..warranty_end_byte]
}
