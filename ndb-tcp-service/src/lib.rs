use std::collections::HashMap;
use std::io::{Cursor, Read};
use serde::{Deserialize, Serialize};
use ndb_core::utils::serde::de_u8_to_bool;

/// Represents a single TCP service entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpServiceEntry {
    pub port: u16,
    pub name: String,
    pub description: Option<String>,
    #[serde(deserialize_with = "de_u8_to_bool")]
    pub wellknown: bool,
    #[serde(deserialize_with = "de_u8_to_bool")]
    pub common: bool,
}

/// Represents the TCP service database
pub struct TcpServiceDb {
    inner: HashMap<u16, TcpServiceEntry>,
}

impl TcpServiceDb {
    /// Load database from a CSV reader
    pub fn from_csv<R: Read>(reader: R) -> Result<Self, csv::Error> {
        let mut rdr = csv::Reader::from_reader(reader);
        let mut map = HashMap::new();
        for result in rdr.deserialize::<TcpServiceEntry>() {
            let entry = result?;
            map.insert(entry.port, entry);
        }
        Ok(Self { inner: map })
    }

    /// Load embedded (bundled) database
    #[cfg(feature = "bundled")]
    pub fn bundled() -> Self {
        static CSV_DATA: &str = include_str!("../data/tcp-services.csv");
        Self::from_csv(Cursor::new(CSV_DATA)).expect("Failed to load bundled tcp-services.csv")
    }

    /// Lookup a TCP service name by port
    pub fn get_name(&self, port: u16) -> Option<&str> {
        self.inner.get(&port).map(|e| e.name.as_str())
    }

    /// Lookup a TCP service entry by port
    pub fn get(&self, port: u16) -> Option<&TcpServiceEntry> {
        self.inner.get(&port)
    }
    
    /// Get all TCP service entries as an iterator
    pub fn all(&self) -> impl Iterator<Item = (&u16, &TcpServiceEntry)> {
        self.inner.iter()
    }

    /// Get well-known TCP service entries
    pub fn wellknown(&self) -> impl Iterator<Item = (&u16, &TcpServiceEntry)> {
        self.inner.iter().filter(|(_, e)| e.wellknown)
    }

    /// Get common TCP service entries
    pub fn common(&self) -> impl Iterator<Item = (&u16, &TcpServiceEntry)> {
        self.inner.iter().filter(|(_, e)| e.common)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "bundled")]
    fn test_lookup_known_port() {
        let db = TcpServiceDb::bundled();
        let entry = db.get(80).expect("Port 80 should exist");
        assert_eq!(entry.name, "http");
        assert!(entry.wellknown);
        assert!(entry.common);
    }

    #[test]
    #[cfg(feature = "bundled")]
    fn test_lookup_unknown_port() {
        let db = TcpServiceDb::bundled();
        assert!(db.get(u16::MAX).is_none());
    }

    #[test]
    #[cfg(feature = "bundled")]
    fn test_get_name() {
        let db = TcpServiceDb::bundled();
        assert_eq!(db.get_name(443), Some("https"));
    }

    #[test]
    #[cfg(feature = "bundled")]
    fn test_iter_wellknown() {
        let db = TcpServiceDb::bundled();
        assert!(db.wellknown().any(|(port, _)| *port == 22));
    }

    #[test]
    #[cfg(feature = "bundled")]
    fn test_iter_common() {
        let db = TcpServiceDb::bundled();
        assert!(db.common().any(|(port, _)| *port == 53));
    }
}
