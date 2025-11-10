# timestamp

Unix timestamp manipulation utility

## Installation

```bash
yay -S timestamp          # Arch Linux (AUR)
cargo install --path .    # From source
```

## Usage

```bash
timestamp
# 1762736502

timestamp 1762736000
# 8m 24s ago

timestamp 1762736000 1762826000
# 1day 1h
```

Automatically detects seconds or milliseconds (`< 10_000_000_000` → seconds, `≥ 10_000_000_000` → milliseconds).
