# checkr

`checkr` is a small Rust service that turns host-level health signals into an HTTP endpoint.  
It loads a JSON configuration, wires up the enabled checkers, and serves their results via `/health`.

> **Platform:** The current implementation reads `/proc/loadavg`, `/proc/meminfo`, `/proc/self/mounts`,
> uses `statvfs`, and shells out to `ss -H -tulnp`, so it targets Linux hosts with the `ss` command
> (from `iproute2`) available. Run it with enough privileges to read those files and list socket owners.

## How it works

- Configuration path comes from `--config/-c` (defaults to `config.json` in the working directory).
- `tokio` + `axum` host a server on the configured `port` and expose a single route: `GET /health`.
- Every time the endpoint is hit, each enabled checker runs (blocking work is moved off the async
  runtime with `spawn_blocking`), and the endpoint responds with the combined results.
- Each result follows `{ "name": "<checker>", "result": "<status>", "descr": "<optional detail>" }`
  where `result` is one of `ok`, `warning`, `critical`, `disabled`, or `error`.

Example response:

```json
[
  { "name": "cpu", "result": "ok", "descr": null },
  { "name": "ram", "result": "warning", "descr": "usage: 82%" },
  {
    "name": "disk_usage",
    "result": "critical",
    "descr": "Disk usage for / is 95.0%"
  }
]
```

## Build and run

```bash
cargo build --release
./target/release/checkr --config config.json
# or during development
cargo run -- --config config.example.json
```

## Configuration reference

`checkr` expects a JSON file. The top-level structure looks like this:

```json
{
  "port": 3000,
  "log_level": "info",
  "cpu": { ... },
  "ram": { ... },
  "disk_usage": { ... },
  "allowed_ports": { ... }
}
```

- `port` (number, required): TCP port to bind the HTTP server to (`0.0.0.0:<port>`).
- `log_level` (string, optional): One of `debug`, `info`, `warn`, `error`, `trace`. Defaults to `error`.
- Each checker block is optional; omit it to skip the checker entirely.
- When a checker block is provided, you can also disable it explicitly with `"enabled": false`.

### CPU load checker

Reads `/proc/loadavg` and compares the 1/5/15-minute load averages against configured thresholds.

```json
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
}
```

All threshold fields must be present if the checker is enabled. A result is `warning` or `critical`
when **any** sampled load average exceeds the configured value.

### RAM usage checker

Parses `/proc/meminfo`, computes percentage usage as `(MemTotal - MemAvailable) / MemTotal * 100`,
and compares it to the provided thresholds (must be present when enabled).

```json
"ram": {
  "enabled": true,
  "warning_threshold": 75.0,
  "critical_threshold": 90.0
}
```

### Disk usage checker

Walks `/proc/self/mounts`, filters out virtual filesystems, and uses `statvfs` to measure usage for
each remaining mount. If any mount crosses the warning or critical threshold, the checker reports it
and lists all offending mounts in the description.

```json
"disk_usage": {
  "enabled": true,
  "warning_threshold": 80.0,
  "critical_threshold": 90.0
}
```

Thresholds must be provided when enabled. Values above the threshold trigger the corresponding state.

### Allowed ports checker

Executes `ss -H -tulnp` and ensures that only the declared processes expose the declared ports.
Anything else is surfaced as `critical` along with a `[process]: port` list.

```json
"allowed_ports": {
  "enabled": true,
  "processes": [
    { "owner": "nginx", "ports": ["80", "443"] },
    { "owner": "node", "ports": ["3000-3999"] }
  ]
}
```

- `owner` must match the process name reported by `ss`.
- `ports` accepts single ports (`"22"`) or inclusive ranges (`"8000-8100"`); whitespace is ignored.
- The checker is disabled unless `enabled` is set to `true` and at least one process entry is supplied.

## Sample configuration

See `config.example.json` for a ready-to-tweak file that enables CPU and RAM checks. Extend it with
`disk_usage` and `allowed_ports` blocks as shown above to turn on those checks.
