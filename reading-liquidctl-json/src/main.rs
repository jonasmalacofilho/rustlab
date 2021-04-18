use colored::Colorize;
use serde_json::Value;
use std::io::{self, Result};
use structopt::StructOpt;

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
fn show_in_sections(statuses: &[lq::Status], with_unit: &Option<String>) {
    let sections = [
        ("Temperature", Some("°C")),
        ("Fan/pump speed", Some("rpm")),
        ("Fan/pump duty cycle", Some("%")),
        ("Voltage", Some("V")),
        ("Current", Some("A")),
        ("Power", Some("W")),
        ("Other", None),
    ];
    let sectioned: Vec<_> = sections.iter().filter_map(|(_, u)| *u).collect();

    let mut first_section = true;
    for (name, sec_unit) in sections.iter() {
        let sectioned = &sectioned;

        let mut sensors = sensors(statuses)
            .filter(move |sensor| match with_unit {
                Some(with_unit) => &sensor.unit == with_unit,
                None => true,
            })
            .filter(move |sensor| match sec_unit {
                Some(sec_unit) => &sensor.unit == sec_unit,
                None => !sectioned.contains(&sensor.unit),
            })
            .peekable();

        if sensors.peek().is_none() {
            continue;
        }

        if first_section {
            first_section = false;
        } else {
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

/// Parse and print JSON sensor data in `stdin` coming from liquidctl.
#[derive(StructOpt)]
struct Opt {
    /// Filter devices on <bus>.
    #[structopt(long)]
    bus: Option<String>,

    /// Filter devices on <address>.
    #[structopt(long)]
    address: Option<String>,

    /// Filter sensors with <unit>.
    #[structopt(long)]
    unit: Option<String>,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let statuses = lq::read_statuses(io::stdin())?;

    if statuses.is_empty() {
        return Ok(());
    }

    let statuses: Vec<_> = statuses
        .into_iter()
        .filter(|x| {
            for (exp, val) in &[(&opt.bus, &x.bus), (&opt.address, &x.address)] {
                if let Some(exp) = exp {
                    if &exp != val {
                        return false;
                    }
                }
            }
            true
        })
        .collect();

    if statuses.is_empty() {
        // nothing to report, but exit with 0 since no devices remained
        std::process::exit(1);
    }

    show_in_sections(&statuses, &opt.unit);

    Ok(())
}
