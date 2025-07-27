use anyhow::Result;
use rangemap::RangeInclusiveMap;
use serde::{Deserialize, Serialize};
use std::{io::Read, net::Ipv4Addr};

pub const CSV_NAME: &str = "ipv4-asn.csv";
pub const BIN_NAME: &str = "ipv4-asn.bin";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Ipv4AsnEntry {
    pub ip_from: u32,
    pub ip_to: u32,
    pub asn: u32,
}

/// Represents the IPv4 ASN database
pub struct Ipv4AsnDb {
    inner_range: RangeInclusiveMap<u32, u32>,
}

impl Ipv4AsnDb {
    /// Load database from a CSV reader
    pub fn from_csv<R: Read>(reader: R) -> Result<Self, csv::Error> {
        let mut rdr = csv::Reader::from_reader(reader);
        let mut range_map: RangeInclusiveMap<u32, u32> = RangeInclusiveMap::new();
        for result in rdr.deserialize::<Ipv4AsnEntry>() {
            let entry = result?;
            range_map.insert(entry.ip_from..=entry.ip_to, entry.asn);
        }
        Ok(Self {
            inner_range: range_map,
        })
    }

    /// Create a new IPv4 ASN database from a vector of entries
    pub fn from_entries(entries: Vec<Ipv4AsnEntry>) -> Self {
        let inner_range = entries
            .into_iter()
            .map(|entry| (entry.ip_from..=entry.ip_to, entry.asn))
            .collect();
        Self { inner_range }
    }

    /// Create a new IPv4 ASN database from a binary slice
    fn from_slice(slice: &[u8]) -> Result<Self> {
        let (entries, _): (Vec<Ipv4AsnEntry>, _) =
            bincode::serde::decode_from_slice(slice, bincode::config::standard())?;
        Ok(Self::from_entries(entries))
    }

    /// Load embedded (bundled) database
    #[cfg(feature = "bundled")]
    pub fn bundled() -> Self {
        static BIN_DATA: &[u8] = include_bytes!("../data/ipv4-asn.bin");
        Self::from_slice(BIN_DATA).expect("Failed to load bundled ipv4-asn.bin")
    }

    /// Get ASN by u32 IP address
    pub fn get(&self, ip: u32) -> Option<&u32> {
        self.inner_range.get(&ip)
    }

    /// Lookup ASN by IPv4 address
    pub fn lookup(&self, ip: Ipv4Addr) -> Option<&u32> {
        let ip_u32 = u32::from(ip);
        self.inner_range.get(&ip_u32)
    }

    /// Get all ASN entries as an iterator
    pub fn all(&self) -> impl Iterator<Item = Ipv4AsnEntry> + '_ {
        self.inner_range.iter().map(|(range, asn)| Ipv4AsnEntry {
            ip_from: *range.start(),
            ip_to: *range.end(),
            asn: *asn,
        })
    }

    pub fn entries(&self) -> Vec<Ipv4AsnEntry> {
        self.all().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_ipv4_asn_lookup() {
        let entries = vec![
            Ipv4AsnEntry {
                ip_from: 167772160, // 10.0.0.0
                ip_to: 167772415,   // 10.0.0.255
                asn: 64500,
            },
            Ipv4AsnEntry {
                ip_from: 3232235520, // 192.168.0.0
                ip_to: 3232235775,   // 192.168.0.255
                asn: 64501,
            },
        ];

        let db = Ipv4AsnDb::from_entries(entries);

        // 10.0.0.1
        let ip1 = Ipv4Addr::new(10, 0, 0, 1);
        assert_eq!(db.lookup(ip1), Some(&64500));

        // 192.168.0.254
        let ip2 = Ipv4Addr::new(192, 168, 0, 254);
        assert_eq!(db.lookup(ip2), Some(&64501));

        // 8.8.8.8 not in DB
        let ip3 = Ipv4Addr::new(8, 8, 8, 8);
        assert_eq!(db.lookup(ip3), None);
    }

    #[test]
    fn test_ipv4_asn_entries_roundtrip() {
        let entries = vec![
            Ipv4AsnEntry {
                ip_from: 100,
                ip_to: 200,
                asn: 1000,
            },
            Ipv4AsnEntry {
                ip_from: 300,
                ip_to: 400,
                asn: 2000,
            },
        ];
        let db = Ipv4AsnDb::from_entries(entries.clone());
        let out_entries = db.entries();

        let mut expected = entries;
        let mut actual = out_entries;
        expected.sort_by_key(|e| e.ip_from);
        actual.sort_by_key(|e| e.ip_from);

        assert_eq!(expected, actual);
    }
}
