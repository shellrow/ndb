use std::collections::HashMap;
use std::io::{Cursor, Read};
use serde::{Deserialize, Serialize};

pub use netdev::MacAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OuiEntry {
    pub mac_prefix: String,
    pub vendor: String,
    pub vendor_detail: Option<String>,
}

pub struct OuiDb {
    inner: HashMap<String, OuiEntry>,
}

impl OuiDb {
    pub fn from_csv<R: Read>(reader: R) -> Result<Self, csv::Error> {
        let mut rdr = csv::Reader::from_reader(reader);
        let mut map = HashMap::new();
        for result in rdr.deserialize::<OuiEntry>() {
            let entry = result?;
            map.insert(entry.mac_prefix.clone(), entry);
        }
        Ok(Self { inner: map })
    }

    #[cfg(feature = "bundled")]
    pub fn bundled() -> Self {
        static CSV_DATA: &str = include_str!("../data/oui.csv");
        Self::from_csv(Cursor::new(CSV_DATA)).expect("Failed to load bundled oui.csv")
    }

    pub fn get(&self, prefix: &str) -> Option<&OuiEntry> {
        self.inner.get(prefix)
    }

    pub fn all(&self) -> impl Iterator<Item = (&String, &OuiEntry)> {
        self.inner.iter()
    }

    /// Lookup from string MAC address (e.g., "ac:4a:56:12:34:56")
    pub fn lookup(&self, mac_str: &str) -> Option<&OuiEntry> {
        let mac = MacAddr::from_hex_format(mac_str);
        self.lookup_mac(mac)
    }

    /// Lookup from `MacAddr` instance
    pub fn lookup_mac(&self, mac: MacAddr) -> Option<&OuiEntry> {
        let octets = mac.octets();

        // Check for exact prefix match
        let key = format!("{:02X}:{:02X}:{:02X}", octets[0], octets[1], octets[2]);
        if let Some(entry) = self.inner.get(&key) {
            return Some(entry);
        }
        // Check for CIDR notation
        for (key, entry) in self.inner.iter() {
            if let Some((prefix, bits)) = parse_mac_prefix_cidr(key) {
                if bitwise_eq(octets, prefix, bits) {
                    return Some(entry);
                }
            }
        }
        None
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

fn bitwise_eq(mac: [u8; 6], prefix: [u8; 6], prefix_len: u8) -> bool {
    let bytes = (prefix_len / 8) as usize;
    let bits = prefix_len % 8;

    if mac[..bytes] != prefix[..bytes] {
        return false;
    }

    if bits > 0 {
        let mask = 0xFF << (8 - bits);
        mac[bytes] & mask == prefix[bytes] & mask
    } else {
        true
    }
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
        let entry = db.lookup_mac(mac);
        assert!(entry.is_some());
    }

    #[test]
    fn test_lookup_mac_cidr() {
        let db = OuiDb::bundled();
        let mac = MacAddr::from_hex_format("fc:d2:b6:0a:00:00");
        let entry = db.lookup_mac(mac);
        assert!(entry.is_some());
    }

    #[test]
    fn test_lookup_mac_str() {
        let db = OuiDb::bundled();
        let entry = db.lookup("ac:4a:56:12:34:56");
        assert!(entry.is_some());
    }

}
