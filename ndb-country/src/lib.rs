use std::collections::HashMap;
use std::io::Read;
use serde::{Deserialize, Serialize};
use anyhow::Result;

pub const CSV_NAME: &str = "country.csv";
pub const BIN_NAME: &str = "country.bin";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CountryEntry {
    pub code: String,
    pub name: String,
}

/// Represents the Country database
pub struct CountryDb {
    inner: HashMap<String, String>,
}

impl CountryDb {
    /// Load database from a CSV reader
    pub fn from_csv<R: Read>(reader: R) -> Result<Self, csv::Error> {
        let mut rdr = csv::Reader::from_reader(reader);
        let mut map = HashMap::new();
        for result in rdr.deserialize::<CountryEntry>() {
            let entry = result?;
            map.insert(entry.code.clone(), entry.name);
        }
        Ok(Self { inner: map })
    }

    /// Create a new Country database from a vector of entries
    pub fn from_entries(entries: Vec<CountryEntry>) -> Self {
        let inner = entries.into_iter().map(|entry| (entry.code, entry.name)).collect();
        Self { inner }
    }

    /// Create a new Country database from a binary slice
    fn from_slice(slice: &[u8]) -> Result<Self> {
        let (entries, _): (Vec<CountryEntry>, _) = bincode::serde::decode_from_slice(slice, bincode::config::standard())?;
        Ok(Self::from_entries(entries))
    }

    /// Load embedded (bundled) database
    #[cfg(feature = "bundled")]
    pub fn bundled() -> Self {
        static BIN_DATA: &[u8] = include_bytes!("../data/country.bin");
        Self::from_slice(BIN_DATA).expect("Failed to load bundled countries.bin")
    }

    pub fn get_name(&self, code: &str) -> Option<&str> {
        self.inner.get(code).map(|name| name.as_str())
    }

    pub fn all(&self) -> impl Iterator<Item = (&String, &String)> {
        self.inner.iter()
    }

    pub fn entries(&self) -> Vec<CountryEntry> {
        self.inner.iter().map(|(code, name)| CountryEntry { code: code.clone(), name: name.clone() }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_country_from_entries_and_lookup() {
        let entries = vec![
            CountryEntry { code: "JP".into(), name: "Japan".into() },
            CountryEntry { code: "US".into(), name: "United States".into() },
        ];

        let db = CountryDb::from_entries(entries);

        assert_eq!(db.get_name("JP"), Some("Japan"));
        assert_eq!(db.get_name("US"), Some("United States"));
        assert_eq!(db.get_name("XX"), None);
    }

    #[test]
    fn test_country_entries_roundtrip() {
        let entries = vec![
            CountryEntry { code: "FR".into(), name: "France".into() },
            CountryEntry { code: "DE".into(), name: "Germany".into() },
        ];

        let db = CountryDb::from_entries(entries.clone());
        let out_entries = db.entries();

        let mut expected = entries;
        let mut actual = out_entries;
        expected.sort_by(|a, b| a.code.cmp(&b.code));
        actual.sort_by(|a, b| a.code.cmp(&b.code));

        assert_eq!(expected, actual);
    }
}
