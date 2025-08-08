use anyhow::Result;
use rangemap::RangeInclusiveMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Read;

pub use netdev::MacAddr;

pub const CSV_NAME: &str = "oui.csv";
pub const BIN_NAME: &str = "oui.bin";

/// Represents a single OUI entry
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OuiEntry {
    pub mac_prefix: String,
    pub vendor: String,
    pub vendor_detail: Option<String>,
}

/// Represents the OUI database
pub struct OuiDb {
    inner: HashMap<String, OuiEntry>,
    inner_range: RangeInclusiveMap<u64, OuiEntry>,
}

impl OuiDb {
    /// Create a new OUI database from a CSV reader
    pub fn from_csv<R: Read>(reader: R) -> Result<Self, csv::Error> {
        let mut rdr = csv::Reader::from_reader(reader);
        let mut map = HashMap::new();
        let mut range_map: RangeInclusiveMap<u64, OuiEntry> = RangeInclusiveMap::new();
        for result in rdr.deserialize::<OuiEntry>() {
            let entry = result?;
            if let Some((prefix, bits)) = parse_mac_prefix_cidr(&entry.mac_prefix) {
                let start = mac_to_u64(prefix) & (!0u64 << (48 - bits));
                let end = start | ((1u64 << (48 - bits)) - 1);
                range_map.insert(start..=end, entry.clone());
            } else {
                map.insert(entry.mac_prefix.clone(), entry);
            }
        }
        Ok(Self {
            inner: map,
            inner_range: range_map,
        })
    }

    /// Create a new OUI database from a vector of entries
    pub fn from_entries(entries: Vec<OuiEntry>) -> Self {
        let mut inner = HashMap::new();
        let mut inner_range = RangeInclusiveMap::new();
        for entry in entries {
            if let Some((prefix, bits)) = parse_mac_prefix_cidr(&entry.mac_prefix) {
                let start = mac_to_u64(prefix) & (!0u64 << (48 - bits));
                let end = start | ((1u64 << (48 - bits)) - 1);
                inner_range.insert(start..=end, entry.clone());
            } else {
                inner.insert(entry.mac_prefix.clone(), entry);
            }
        }
        Self { inner, inner_range }
    }

    /// Create a new OUI database from a binary slice
    fn from_slice(slice: &[u8]) -> Result<Self> {
        let (entries, _): (Vec<OuiEntry>, _) =
            bincode::serde::decode_from_slice(slice, bincode::config::standard())?;
        Ok(Self::from_entries(entries))
    }

    /// Create a new OUI database from a bundled file
    #[cfg(feature = "bundled")]
    pub fn bundled() -> Self {
        static BIN_DATA: &[u8] = include_bytes!("../data/oui.bin");
        Self::from_slice(BIN_DATA).expect("Failed to load bundled oui.bin")
    }

    /// Get an OUI entry by its MAC prefix.
    /// Use `lookup` or `lookup_mac` for more flexible lookups
    pub fn get(&self, prefix: &str) -> Option<&OuiEntry> {
        self.inner.get(prefix)
    }

    /// Get all OUI entries as an iterator
    pub fn all(&self) -> impl Iterator<Item = (&String, &OuiEntry)> {
        self.inner.iter()
    }

    /// Lookup from string MAC address (e.g., "ac:4a:56:12:34:56")
    pub fn lookup(&self, mac_str: &str) -> Option<&OuiEntry> {
        let mac = MacAddr::from_hex_format(mac_str);
        self.lookup_mac(&mac)
    }

    /// Lookup from `MacAddr` instance
    pub fn lookup_mac(&self, mac: &MacAddr) -> Option<&OuiEntry> {
        let octets = mac.octets();

        // Range (CIDR) match
        let mac_u64 = mac_to_u64(octets);
        if let Some(entry) = self.inner_range.get(&mac_u64) {
            return Some(entry);
        }

        // Exact match
        let key = format!("{:02X}:{:02X}:{:02X}", octets[0], octets[1], octets[2]);
        self.inner.get(&key)
    }

    /// Get all entries as a vector
    pub fn entries(&self) -> Vec<OuiEntry> {
        self.inner.values().cloned().collect()
    }
}

fn parse_mac_prefix_cidr(s: &str) -> Option<([u8; 6], u8)> {
    let parts: Vec<&str> = s.split('/').collect();
    if parts.len() != 2 {
        return None;
    }
    let mac = MacAddr::from_hex_format(parts[0]);
    let bits = parts[1].parse::<u8>().ok()?;
    Some((mac.octets(), bits))
}

fn mac_to_u64(mac: [u8; 6]) -> u64 {
    ((mac[0] as u64) << 40)
        | ((mac[1] as u64) << 32)
        | ((mac[2] as u64) << 24)
        | ((mac[3] as u64) << 16)
        | ((mac[4] as u64) << 8)
        | (mac[5] as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_known_mac_prefix() {
        let db = OuiDb::bundled();
        let entry = db.get("AC:4A:56").expect("Known prefix should exist");
        assert_eq!(entry.vendor, "Cisco");
        assert!(entry.vendor_detail.is_some());
    }

    #[test]
    fn test_lookup_unknown_mac_prefix() {
        let db = OuiDb::bundled();
        assert!(db.get("FF:FF:FF").is_none());
    }

    #[test]
    fn test_all_contains_entries() {
        let db = OuiDb::bundled();
        assert!(db.all().count() > 100);
    }

    #[test]
    fn test_lookup_mac_exact() {
        let db = OuiDb::bundled();
        let mac = MacAddr::from_hex_format("ac:4a:56:12:34:56");
        let entry = db.lookup_mac(&mac);
        assert!(entry.is_some());
    }

    #[test]
    fn test_lookup_mac_cidr() {
        let db = OuiDb::bundled();
        let mac = MacAddr::from_hex_format("fc:d2:b6:0a:00:00");
        let entry = db.lookup_mac(&mac);
        assert!(entry.is_some());
    }

    #[test]
    fn test_lookup_mac_str() {
        let db = OuiDb::bundled();
        let entry = db.lookup("ac:4a:56:12:34:56");
        assert!(entry.is_some());
    }
}
