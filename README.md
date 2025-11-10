# timestamp

Timestamp utility for unix timestamps.

## Installation

```bash
cargo install --path .
```

Or build release binary:

```bash
cargo build --release
# Binary at ./target/release/ts
```

## Usage

### Get current timestamp

```bash
ts
# 1762736502
```

### Time since timestamp

```bash
ts 1762736000
# 8m 24s ago

ts 9999999999
# 261years 8days 10h 44m 52s in the future
```

### Difference between timestamps

```bash
ts 1762736000 1762826000
# 1day 1h
```

### Timestamp formats

Automatically detects seconds or milliseconds based on magnitude:
- `< 10_000_000_000` → seconds
- `≥ 10_000_000_000` → milliseconds

```bash
ts 1762736000      # seconds
ts 1762736000000   # milliseconds
```

## Help

```bash
ts --help
```
