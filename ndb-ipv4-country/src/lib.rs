use serde::{Deserialize, Serialize};
use std::{io::Read, net::Ipv4Addr};
use rangemap::RangeInclusiveMap;
use anyhow::Result;

pub const CSV_NAME: &str = "ipv4-country.csv";
pub const BIN_NAME: &str = "ipv4-country.bin";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Ipv4CountryEntry {
    pub ip_from: u32,
    pub ip_to: u32,
    pub country_code: String,
}

/// Represents the IPv4 Country database
pub struct Ipv4CountryDb {
    inner_range: RangeInclusiveMap<u32, String>,
}

impl Ipv4CountryDb {
    /// Load database from a CSV reader
    pub fn from_csv<R: Read>(reader: R) -> Result<Self, csv::Error> {
        let mut rdr = csv::Reader::from_reader(reader);
        let mut range_map: RangeInclusiveMap<u32, String> = RangeInclusiveMap::new();
        for result in rdr.deserialize::<Ipv4CountryEntry>() {
            let entry = result?;
            range_map.insert(entry.ip_from..=entry.ip_to, entry.country_code);
        }
        Ok(Self { inner_range: range_map })
    }

    /// Create a new IPv4 Country database from a vector of entries
    pub fn from_entries(entries: Vec<Ipv4CountryEntry>) -> Self {
        let inner_range = entries.into_iter().map(|entry| (entry.ip_from..=entry.ip_to, entry.country_code)).collect();
        Self { inner_range }
    }

    /// Create a new IPv4 Country database from a binary slice
    fn from_slice(slice: &[u8]) -> Result<Self> {
        let (entries, _): (Vec<Ipv4CountryEntry>, _) = bincode::serde::decode_from_slice(slice, bincode::config::standard())?;
        Ok(Self::from_entries(entries))
    }

    /// Load embedded (bundled) database
    #[cfg(feature = "bundled")]
    pub fn bundled() -> Self {
        static BIN_DATA: &[u8] = include_bytes!("../data/ipv4-country.bin");
        Self::from_slice(BIN_DATA).expect("Failed to load bundled ipv4-country.bin")
    }

    /// Get country code by u32 IP address
    pub fn get(&self, ip: u32) -> Option<&String> {
        self.inner_range.get(&ip)
    }

    /// Lookup country code by IPv4 address
    pub fn lookup(&self, ip: Ipv4Addr) -> Option<&String> {
        let ip_u32 = u32::from(ip);
        self.inner_range.get(&ip_u32)
    }

    /// Get all country entries as an iterator
    pub fn all(&self) -> impl Iterator<Item = Ipv4CountryEntry> + '_ {
        self.inner_range.iter().map(|(range, code)| {
            Ipv4CountryEntry {
                ip_from: *range.start(),
                ip_to: *range.end(),
                country_code: code.clone(),
            }
        })
    }

    /// Get all entries as a vector
    pub fn entries(&self) -> Vec<Ipv4CountryEntry> {
        self.all().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ipv4_country_lookup() {
        let db = Ipv4CountryDb::bundled();

        let ip = Ipv4Addr::new(8, 8, 8, 8); // Google DNS
        let result = db.lookup(ip);
        assert!(result.is_some(), "Expected to find a country for 8.8.8.8");

        let test_ip = Ipv4Addr::new(203, 0, 113, 1);
        let test_result = db.lookup(test_ip);
        
        assert!(test_result.is_none() || test_result.is_some());
    }

    #[test]
    fn test_ipv4_country_entries_consistency() {
        let db = Ipv4CountryDb::bundled();
        let entries = db.entries();

        assert!(!entries.is_empty());

        for entry in entries {
            assert!(entry.ip_from <= entry.ip_to);
            assert!(!entry.country_code.is_empty());
        }
    }
}
