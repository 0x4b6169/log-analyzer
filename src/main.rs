extern crate core;

mod sigma_rule;
mod yml;
mod detection;

use anyhow::Error;
use sigma_rule::SigmaRule;
use crate::detection::Detection;

// Main should...
// N/A    0. Read a config file in case path is different than defaults (for rules, field mappings, kafka/http/etc props)
// DONE - 1. Add all rules
// N/A    2. Add field mappings
// PROG   3. Create detections from
// N/A    4. Begin loop of processing requests (start with simple rules, not aggregate until able to back with Kafka / Redis / Elastic)
// N/A    5. Within loop, begin async concurrent processing of sigma rules in memory

fn main() -> Result<(), Error> {
    let sigma_rules = SigmaRule::process_sigma_rules("config/rules".to_string())?;
    let nice = Detection::process_detection(sigma_rules);

    Ok(())
}


