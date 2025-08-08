# checkr

A simple CLI tool to monitor key Linux system metrics including CPU load, RAM usage, and more.

Useful for lightweight system monitoring or embedding into other tooling and automation setups.

## Features

- View current CPU load averages (1, 5, 15 min)
- Monitor available RAM as a percentage
- Define warning and critical thresholds
- Optional logging and config via file
- Easily extendable with other checks (e.g., disk usage, open ports, etc.)

## Installation

Build with Cargo:

```bash
cargo build --release
```

The binary will be located at target/release/checkr.

## Usage

Run checkr with a path to your config file:

```bash
./checkr --config path/to/config.json
```

If not specified, it defaults to config.json in the current directory.

### Command-line options

```bash
--config, -c    Path to the configuration file [default: config.json]
```

## Configuration

The config file must be in JSON format and supports the following structure:

```json
{
  "port": 8080,
  "log_level": "info",
  "cpu": {
    "enabled": true,
    "warning": {
      "one_threshold": 1.0,
      "five_threshold": 0.8,
      "fifteen_threshold": 0.6
    },
    "critical": {
      "one_threshold": 2.0,
      "five_threshold": 1.5,
      "fifteen_threshold": 1.0
    }
  },
  "ram": {
    "enabled": true,
    "warning_threshold": 30.0,
    "critical_threshold": 10.0
  }
}
```
### Explanation

- port: Port to expose the service (if applicable).
- log_level: Optional log level (debug, info, warn, error, trace)
- cpu: CPU check configuration.
    - enabled: Whether to enable CPU check.
    - warning/critical: Thresholds for 1, 5, and 15 minute CPU load averages.
- ram: RAM check configuration.
    - enabled: Whether to enable RAM check.
    - warning_threshold / critical_threshold: Free memory percentage thresholds.

#### Notes

- CPU thresholds are based on the load average (number of runnable processes) over time.
- RAM thresholds are percentages of available memory.
