use anyhow::Result;
use rangemap::RangeInclusiveMap;
use serde::{Deserialize, Serialize};
use std::{io::Read, net::Ipv6Addr};

pub const CSV_NAME: &str = "ipv6-country.csv";
pub const BIN_NAME: &str = "ipv6-country.bin";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Ipv6CountryEntry {
    pub ip_from: u128,
    pub ip_to: u128,
    pub country_code: String,
}

/// Represents the IPv6 Country database
pub struct Ipv6CountryDb {
    inner_range: RangeInclusiveMap<u128, String>,
}

impl Ipv6CountryDb {
    /// Load database from a CSV reader
    pub fn from_csv<R: Read>(reader: R) -> Result<Self, csv::Error> {
        let mut rdr = csv::Reader::from_reader(reader);
        let mut range_map: RangeInclusiveMap<u128, String> = RangeInclusiveMap::new();
        for result in rdr.deserialize::<Ipv6CountryEntry>() {
            let entry = result?;
            range_map.insert(entry.ip_from..=entry.ip_to, entry.country_code);
        }
        Ok(Self {
            inner_range: range_map,
        })
    }

    /// Create a new IPv6 Country database from a vector of entries
    pub fn from_entries(entries: Vec<Ipv6CountryEntry>) -> Self {
        let inner_range = entries
            .into_iter()
            .map(|entry| (entry.ip_from..=entry.ip_to, entry.country_code))
            .collect();
        Self { inner_range }
    }

    /// Create a new IPv6 Country database from a binary slice
    fn from_slice(slice: &[u8]) -> Result<Self> {
        let (entries, _): (Vec<Ipv6CountryEntry>, _) =
            bincode::serde::decode_from_slice(slice, bincode::config::standard())?;
        Ok(Self::from_entries(entries))
    }

    /// Load embedded (bundled) database
    #[cfg(feature = "bundled")]
    pub fn bundled() -> Self {
        static BIN_DATA: &[u8] = include_bytes!("../data/ipv6-country.bin");
        Self::from_slice(BIN_DATA).expect("Failed to load bundled ipv6-country.bin")
    }

    /// Get country code by u128 IP address
    pub fn get(&self, ip: u128) -> Option<&String> {
        self.inner_range.get(&ip)
    }

    /// Lookup country code by IPv6 address
    pub fn lookup(&self, ip: &Ipv6Addr) -> Option<&String> {
        let ip_u128 = u128::from(*ip);
        self.inner_range.get(&ip_u128)
    }

    /// Get all country entries as an iterator
    pub fn all(&self) -> impl Iterator<Item = Ipv6CountryEntry> + '_ {
        self.inner_range
            .iter()
            .map(|(range, code)| Ipv6CountryEntry {
                ip_from: *range.start(),
                ip_to: *range.end(),
                country_code: code.clone(),
            })
    }

    /// Get all entries as a vector
    pub fn entries(&self) -> Vec<Ipv6CountryEntry> {
        self.all().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv6_country_lookup() {
        let db = Ipv6CountryDb::bundled();
        // Cloudflare
        let sample_ip: Ipv6Addr = "2606:4700:4700::1111".parse().unwrap();
        let ip_u128 = u128::from(sample_ip);

        let result = db.get(ip_u128);
        assert!(result.is_some(), "Expected a result for Cloudflare IP");
    }

    #[test]
    fn test_ipv6_country_all_entries() {
        let db = Ipv6CountryDb::bundled();
        let entries: Vec<_> = db.all().collect();
        assert!(!entries.is_empty(), "entries should not be empty");
    }

    #[test]
    fn test_ipv6_country_lookup_str() {
        let db = Ipv6CountryDb::bundled();
        let sample_ip: Ipv6Addr = "::1".parse().unwrap(); // loopback
        let result = db.lookup(&sample_ip);
        println!("Lookup ::1 => {:?}", result);
    }
}
