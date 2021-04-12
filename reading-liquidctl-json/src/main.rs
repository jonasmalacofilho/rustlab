use colored::Colorize;
use serde_json::Value;
use std::io::{self, Result};

/// Definitions and utilities for reading liquidctl `--json` output.
mod lq {
    use super::*;
    use serde::Deserialize;

    /// liquidctl `--json` device status object.
    #[derive(Deserialize, Debug)]
    pub struct Status {
        pub bus: String,
        pub address: String,
        pub description: String,
        pub status: Vec<Triple>,
    }

    /// liquidctl `--json` status item triple.
    #[derive(Deserialize, Debug)]
    pub struct Triple(pub String, pub Value, pub String);

    /// Read a collection of `Status` from `reader`.
    pub fn read_statuses(reader: impl io::Read) -> Result<Vec<Status>> {
        let statuses = serde_json::from_reader(reader)?;
        Ok(statuses)
    }
}

/// Unified view of a sensor that conveniently also includes information about the parent device.
struct Sensor<'a> {
    bus: &'a str,
    address: &'a str,
    description: &'a str,
    key: &'a str,
    value: &'a Value,
    unit: &'a str,
}

/// Return an iterator of unified `Sensor` values from liquidctl `statuses`.
fn sensors<'a>(statuses: &'a [lq::Status]) -> impl Iterator<Item = Sensor<'a>> {
    statuses.iter().flat_map(|dev| {
        dev.status
            .iter()
            .map(move |lq::Triple(key, value, unit)| Sensor {
                bus: &dev.bus,
                address: &dev.address,
                description: &dev.description,
                key,
                value,
                unit,
            })
    })
}

/// Print all sensors in categories
fn show_in_sections(statuses: &[lq::Status]) {
    let sections = [
        ("Temperature", Some("°C")),
        ("Fan/pump speed", Some("rpm")),
        ("Fan/pump duty cycle", Some("%")),
        ("Voltage", Some("V")),
        ("Current", Some("A")),
        ("Power", Some("W")),
        ("Other", None),
    ];

    let not_other: Vec<_> = sections.iter().filter_map(|(_, u)| *u).collect();
    let not_other = &not_other;

    for (i, (name, unit)) in sections.iter().enumerate() {
        let mut sensors = sensors(statuses)
            .filter(move |sensor| match unit {
                Some(unit) => &sensor.unit == unit,
                None => !not_other.contains(&sensor.unit),
            })
            .peekable();

        if sensors.peek().is_none() {
            continue;
        }

        if i > 0 {
            print!("\n");
        }

        println!("{}{}", name.bold(), " readings:".bold());
        sensors.for_each(|sensor| {
            println!(
                "{}:{} / {} / {}: {} {}",
                sensor.bus.cyan().dimmed(),
                sensor.address.cyan().dimmed(),
                sensor.description.green(),
                sensor.key.bold(),
                sensor.value,
                sensor.unit,
            );
        });
    }
}

fn main() -> Result<()> {
    let statuses = lq::read_statuses(io::stdin())?;

    // TODO output only specific devices/keys received as CLI args

    // else just output the most common sections...
    show_in_sections(&statuses);

    Ok(())
}
