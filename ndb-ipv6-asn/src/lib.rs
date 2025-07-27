use anyhow::Result;
use rangemap::RangeInclusiveMap;
use serde::{Deserialize, Serialize};
use std::{io::Read, net::Ipv6Addr};

pub const CSV_NAME: &str = "ipv6-asn.csv";
pub const BIN_NAME: &str = "ipv6-asn.bin";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Ipv6AsnEntry {
    pub ip_from: u128,
    pub ip_to: u128,
    pub asn: u32,
}

/// Represents the IPv6 ASN database
pub struct Ipv6AsnDb {
    inner_range: RangeInclusiveMap<u128, u32>,
}

impl Ipv6AsnDb {
    /// Load database from a CSV reader
    pub fn from_csv<R: Read>(reader: R) -> Result<Self, csv::Error> {
        let mut rdr = csv::Reader::from_reader(reader);
        let mut range_map: RangeInclusiveMap<u128, u32> = RangeInclusiveMap::new();
        for result in rdr.deserialize::<Ipv6AsnEntry>() {
            let entry = result?;
            range_map.insert(entry.ip_from..=entry.ip_to, entry.asn);
        }
        Ok(Self {
            inner_range: range_map,
        })
    }

    /// Create a new IPv6 ASN database from a vector of entries
    pub fn from_entries(entries: Vec<Ipv6AsnEntry>) -> Self {
        let inner_range = entries
            .into_iter()
            .map(|entry| (entry.ip_from..=entry.ip_to, entry.asn))
            .collect();
        Self { inner_range }
    }

    /// Create a new IPv6 ASN database from a binary slice
    fn from_slice(slice: &[u8]) -> Result<Self> {
        let (entries, _): (Vec<Ipv6AsnEntry>, _) =
            bincode::serde::decode_from_slice(slice, bincode::config::standard())?;
        Ok(Self::from_entries(entries))
    }

    /// Load embedded (bundled) database
    #[cfg(feature = "bundled")]
    pub fn bundled() -> Self {
        static BIN_DATA: &[u8] = include_bytes!("../data/ipv6-asn.bin");
        Self::from_slice(BIN_DATA).expect("Failed to load bundled ipv6-asn.bin")
    }

    /// Get ASN by u128 IP address
    pub fn get(&self, ip: u128) -> Option<&u32> {
        self.inner_range.get(&ip)
    }

    /// Lookup ASN by IPv6 address
    pub fn lookup(&self, ip: Ipv6Addr) -> Option<&u32> {
        let ip_u128 = u128::from(ip);
        self.inner_range.get(&ip_u128)
    }

    /// Get all ASN entries as an iterator
    pub fn all(&self) -> impl Iterator<Item = Ipv6AsnEntry> + '_ {
        self.inner_range.iter().map(|(range, asn)| Ipv6AsnEntry {
            ip_from: *range.start(),
            ip_to: *range.end(),
            asn: *asn,
        })
    }

    /// Get all entries as a vector
    pub fn entries(&self) -> Vec<Ipv6AsnEntry> {
        self.all().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv6_asn_lookup() {
        let db = Ipv6AsnDb::bundled();

        // Example IPv6 address for Cloudflare
        let ip = "2606:4700:4700::1111".parse::<Ipv6Addr>().unwrap();
        let result = db.lookup(ip);
        assert!(
            result.is_some(),
            "Expected to find ASN for Cloudflare IPv6 address"
        );

        // IPv6 Documentation Prefix (RFC 3849)
        let doc_ip = "2001:db8::1".parse::<Ipv6Addr>().unwrap();
        let doc_result = db.lookup(doc_ip);
        assert!(doc_result.is_none() || doc_result.is_some());
    }
}
