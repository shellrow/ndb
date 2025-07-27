use std::collections::HashMap;
use std::io::Read;
use serde::{Deserialize, Serialize};
use anyhow::Result;

pub const CSV_NAME: &str = "as.csv";
pub const BIN_NAME: &str = "as.bin";

/// Represents a single Autonomous System (AS) entry
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AsEntry {
    pub asn: u32,
    pub name: String,
}

/// Represents the AS database
pub struct AsDb {
    inner: HashMap<u32, String>,
}

impl AsDb {
    /// Load database from a CSV reader
    pub fn from_csv<R: Read>(reader: R) -> Result<Self, csv::Error> {
        let mut rdr = csv::Reader::from_reader(reader);
        let mut map = HashMap::new();
        for result in rdr.deserialize::<AsEntry>() {
            let entry = result?;
            map.insert(entry.asn, entry.name);
        }
        Ok(Self { inner: map })
    }

    /// Create a new AS database from a vector of entries
    pub fn from_entries(entries: Vec<AsEntry>) -> Self {
        let inner = entries.into_iter().map(|entry| (entry.asn, entry.name)).collect();
        Self { inner }
    }

    /// Create a new AS database from a binary slice
    fn from_slice(slice: &[u8]) -> Result<Self> {
        let (entries, _): (Vec<AsEntry>, _) = bincode::serde::decode_from_slice(slice, bincode::config::standard())?;
        Ok(Self::from_entries(entries))
    }

    /// Load embedded (bundled) database
    #[cfg(feature = "bundled")]
    pub fn bundled() -> Self {
        static BIN_DATA: &[u8] = include_bytes!("../data/as.bin");
        Self::from_slice(BIN_DATA).expect("Failed to load bundled AS database")
    }

    pub fn get_name(&self, asn: u32) -> Option<&str> {
        self.inner.get(&asn).map(|name| name.as_str())
    }

    pub fn all(&self) -> impl Iterator<Item = (&u32, &String)> {
        self.inner.iter()
    }

    pub fn entries(&self) -> Vec<AsEntry> {
        self.inner.iter().map(|(&asn, name)| AsEntry { asn, name: name.clone() }).collect()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const CSV_DATA: &str = "asn,name\n13335,Cloudflare\n15169,Google LLC";

    #[test]
    fn test_from_csv() {
        let reader = Cursor::new(CSV_DATA);
        let db = AsDb::from_csv(reader).expect("CSV parsing failed");

        assert_eq!(db.get_name(13335), Some("Cloudflare"));
        assert_eq!(db.get_name(15169), Some("Google LLC"));
        assert_eq!(db.get_name(99999), None);
    }

    #[test]
    fn test_from_entries_and_lookup() {
        let entries = vec![
            AsEntry { asn: 64500, name: "TestNet".into() },
            AsEntry { asn: 64501, name: "ExampleNet".into() },
        ];
        let db = AsDb::from_entries(entries);
        assert_eq!(db.get_name(64500), Some("TestNet"));
    }

    #[test]
    fn test_serialization_roundtrip() {
        let entries = vec![
            AsEntry { asn: 64500, name: "TestNet".into() },
        ];

        let mut buf = Vec::new();
        bincode::serde::encode_into_std_write(&entries, &mut buf, bincode::config::standard()).unwrap();
        let decoded = AsDb::from_slice(&buf).unwrap();

        assert_eq!(decoded.get_name(64500), Some("TestNet"));
    }
}
