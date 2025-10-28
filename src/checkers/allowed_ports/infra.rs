use crate::checkers::allowed_ports::OpenPortsSource;
use std::{fs, io};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PortEntry(pub u16);

impl PortEntry {
    pub fn port(&self) -> u16 {
        self.0
    }
}

impl From<u16> for PortEntry {
    fn from(port: u16) -> PortEntry {
        PortEntry(port)
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to parse {0} file: {1}")]
pub struct ProcNetParseError(pub String, pub String);

#[derive(thiserror::Error, Debug)]
#[error("Failed to read /proc/net/{0} file: {1}")]
pub struct ProcNetReadFileError(pub String, pub String);

pub struct ProcNetSource;

impl ProcNetSource {
    fn read_proc_ports_file(&self, path: &str) -> Result<String, ProcNetReadFileError> {
        match fs::read_to_string(path) {
            Ok(content) => Ok(content),
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(String::new()), // e.g., ipv6 disabled
            Err(e) => Err(ProcNetReadFileError(path.to_string(), e.to_string())),
        }
    }

    fn parse_proc_ports_single_line(
        &self,
        path: &str,
        line: &str,
        is_tcp: bool,
    ) -> Result<Option<PortEntry>, ProcNetParseError> {
        let mut columns = line.split_whitespace();

        let _sl = columns.next();
        let local_address = match columns.next() {
            Some(s) => s,
            None => {
                return Err(ProcNetParseError(
                    path.to_string(),
                    format!("Can't get local address: {}", line),
                ));
            }
        };

        let _rem_address = columns.next();
        let state = match columns.next() {
            Some(s) => s,
            None => {
                return Err(ProcNetParseError(
                    path.to_string(),
                    format!("Can't get current state: {}", line),
                ));
            }
        };

        // For TCP, we keep only ports in LISTEN (0A) state
        if is_tcp && state != "0A" {
            return Ok(None);
        }

        if let Some((_addr_hex, port_hex)) = local_address.rsplit_once(':') {
            return match u16::from_str_radix(port_hex, 16) {
                Ok(port) => Ok(Some(port.into())),
                Err(e) => Err(ProcNetParseError(
                    path.to_string(),
                    format!("Wrong port value: {}", line),
                )),
            };
        }

        Err(ProcNetParseError(
            path.to_string(),
            format!("Wrong local address: {}", line),
        ))
    }

    ///
    /// Parse a single `/proc/net/*` socket table and return matching open ports.
    ///
    /// This function understands the common column layout used by:
    /// - `/proc/net/tcp`  (TCP IPv4)
    /// - `/proc/net/tcp6` (TCP IPv6)
    /// - `/proc/net/udp`  (UDP IPv4)
    /// - `/proc/net/udp6` (UDP IPv6)
    ///
    /// ### File format (per line)
    /// Each file begins with a header row:
    /// ```text
    /// sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode ...
    /// ```
    /// Followed by rows like (TCP example):
    /// ```text
    ///  0: 0100007F:1F90 00000000:0000 0A 00000000:00000000 00:00000000 00000000  1000 0 18805 ...
    ///      ^            ^             ^
    ///      sl           local         st
    /// ```
    /// Important columns for this parser:
    /// - **local_address**: `IP_HEX:PORT_HEX` (IPv4: 8 hex nibbles; IPv6: 32 hex nibbles)
    /// - **st** (state):
    ///   - TCP: only entries with `st == "0A"` (LISTEN) are considered “open” for server sockets
    ///   - UDP: the state is not meaningful (often `07`); the presence of a row implies the socket is open
    ///
    /// ### Parameters
    /// - `path`: Path to one of `/proc/net/{tcp,tcp6,udp,udp6}`.
    /// - `is_tcp`: Set `true` for TCP files (`tcp`, `tcp6`) to apply `LISTEN (0A)` filtering; `false` for UDP.
    ///
    /// ### Returns
    /// - `Ok(Vec<PortEntry>)` with one entry per discovered open/listening socket (by local port).
    /// - `Err(io::Error)` for unexpected I/O errors.
    ///
    fn parse_proc_ports(&self, path: &str, is_tcp: bool) -> anyhow::Result<Vec<PortEntry>> {
        let content = self.read_proc_ports_file(path)?;

        if content.trim().is_empty() {
            return Ok(Vec::new());
        }

        let mut result = Vec::new();

        for (line_number, line) in content.lines().enumerate() {
            // skip header/empty
            if line_number == 0 || line.trim().is_empty() {
                continue;
            }

            if let Some(port) = self.parse_proc_ports_single_line(path, &line, is_tcp)? {
                result.push(port);
            }
        }

        Ok(result)
    }
}

impl OpenPortsSource for ProcNetSource {
    fn parse_values(&self) -> anyhow::Result<Vec<PortEntry>> {
        let mut result = Vec::new();
        result.extend(self.parse_proc_ports("/proc/net/tcp", true)?);
        result.extend(self.parse_proc_ports("/proc/net/tcp6", true)?);
        result.extend(self.parse_proc_ports("/proc/net/udp", false)?);
        result.extend(self.parse_proc_ports("/proc/net/udp6", false)?);
        Ok(result)
    }
}
