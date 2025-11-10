use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use clap::Parser;
use std::time::Duration;

#[derive(Parser)]
#[command(
    name = "timestamp",
    about = "timestamp operations",
    long_about = "\
Timestamps operations:
  timestamp                    - current timestamp
  timestamp <timestamp>        - time since timestamp
  timestamp <ts1> <ts2>        - difference between timestamps"
)]
struct Args {
    timestamps: Vec<i64>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.timestamps.as_slice() {
        [] => print_current(),
        [ts] => print_since(*ts)?,
        [ts1, ts2] => print_diff(*ts1, *ts2)?,
        _ => anyhow::bail!("too many arguments"),
    }

    Ok(())
}

fn print_current() {
    println!("{}", Utc::now().timestamp());
}

fn print_since(timestamp: i64) -> Result<()> {
    let then = parse_timestamp(timestamp)?;
    let now = Utc::now();
    let seconds_diff = now.timestamp() - then.timestamp();

    let (duration, suffix) = if seconds_diff < 0 {
        (Duration::from_secs((-seconds_diff) as u64), "in the future")
    } else {
        (Duration::from_secs(seconds_diff as u64), "ago")
    };

    println!("{} {}", humantime::format_duration(duration), suffix);
    Ok(())
}

fn print_diff(ts1: i64, ts2: i64) -> Result<()> {
    let t1 = parse_timestamp(ts1)?;
    let t2 = parse_timestamp(ts2)?;
    let seconds_diff = (t2.timestamp() - t1.timestamp()).abs() as u64;
    let duration = Duration::from_secs(seconds_diff);

    println!("{}", humantime::format_duration(duration));
    Ok(())
}

fn parse_timestamp(ts: i64) -> Result<DateTime<Utc>> {
    const SECONDS_THRESHOLD: i64 = 10_000_000_000;

    if ts >= SECONDS_THRESHOLD {
        DateTime::from_timestamp_millis(ts).context("invalid timestamp")
    } else {
        DateTime::from_timestamp(ts, 0).context("invalid timestamp")
    }
}
