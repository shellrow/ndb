use anyhow::Result;
use std::fs::{self, File};
use std::path::PathBuf;
use crate::commands::AppConfig;

pub fn update_bin_db(config: AppConfig) -> Result<()> {
    // Enumrate through the input directory and process files
    for entry in config.input_dir.read_dir()? {
        let entry = entry?;
        match entry.file_name().to_str().unwrap_or_default() {
            ndb_as::CSV_NAME => {
                // Process AS CSV file
                tracing::info!("Processing AS file: {}", entry.path().display());
                let file = File::open(entry.path())
                    .map_err(|e| anyhow::anyhow!("Failed to open AS CSV file: {}", e))?;
                let db = ndb_as::AsDb::from_csv(file)
                    .map_err(|e| anyhow::anyhow!("Failed to process AS CSV: {}", e))?;
                let bin_path = config.output_dir.join(ndb_as::BIN_NAME);
                save_bin(db.entries(), bin_path, config.dry_run)?;
                tracing::info!("AS database updated successfully.");
            }
            ndb_country::CSV_NAME => {
                // Process Country CSV file
                tracing::info!("Processing Country file: {}", entry.path().display());
                let file = File::open(entry.path())
                    .map_err(|e| anyhow::anyhow!("Failed to open Country CSV file: {}", e))?;
                let db = ndb_country::CountryDb::from_csv(file)
                    .map_err(|e| anyhow::anyhow!("Failed to process Country CSV: {}", e))?;
                let bin_path = config.output_dir.join(ndb_country::BIN_NAME);
                save_bin(db.entries(), bin_path, config.dry_run)?;
                tracing::info!("Country database updated successfully.");
            }
            ndb_ipv4_asn::CSV_NAME => {
                // Process IPv4 ASN CSV file
                tracing::info!("Processing IPv4 ASN file: {}", entry.path().display());
                let file = File::open(entry.path())
                    .map_err(|e| anyhow::anyhow!("Failed to open IPv4 ASN CSV file: {}", e))?;
                let db = ndb_ipv4_asn::Ipv4AsnDb::from_csv(file)
                    .map_err(|e| anyhow::anyhow!("Failed to process IPv4 ASN CSV: {}", e))?;
                let bin_path = config.output_dir.join(ndb_ipv4_asn::BIN_NAME);
                save_bin(db.entries(), bin_path, config.dry_run)?;
                tracing::info!("IPv4 ASN database updated successfully.");
            }
            ndb_ipv4_country::CSV_NAME => {
                // Process IPv4 Country CSV file
                tracing::info!("Processing IPv4 Country file: {}", entry.path().display());
                let file = File::open(entry.path())
                    .map_err(|e| anyhow::anyhow!("Failed to open IPv4 Country CSV file: {}", e))?;
                let db = ndb_ipv4_country::Ipv4CountryDb::from_csv(file)
                    .map_err(|e| anyhow::anyhow!("Failed to process IPv4 Country CSV: {}", e))?;
                let bin_path = config.output_dir.join(ndb_ipv4_country::BIN_NAME);
                save_bin(db.entries(), bin_path, config.dry_run)?;
                tracing::info!("IPv4 Country database updated successfully.");
            }
            ndb_ipv6_asn::CSV_NAME => {
                // Process IPv6 ASN CSV file
                tracing::info!("Processing IPv6 ASN file: {}", entry.path().display());
                let file = File::open(entry.path())
                    .map_err(|e| anyhow::anyhow!("Failed to open IPv6 ASN CSV file: {}", e))?;
                let db = ndb_ipv6_asn::Ipv6AsnDb::from_csv(file)
                    .map_err(|e| anyhow::anyhow!("Failed to process IPv6 ASN CSV: {}", e))?;
                let bin_path = config.output_dir.join(ndb_ipv6_asn::BIN_NAME);
                save_bin(db.entries(), bin_path, config.dry_run)?;
                tracing::info!("IPv6 ASN database updated successfully.");
            }
            ndb_ipv6_country::CSV_NAME => {
                // Process IPv6 Country CSV file
                tracing::info!("Processing IPv6 Country file: {}", entry.path().display());
                let file = File::open(entry.path())
                    .map_err(|e| anyhow::anyhow!("Failed to open IPv6 Country CSV file: {}", e))?;
                let db = ndb_ipv6_country::Ipv6CountryDb::from_csv(file)
                    .map_err(|e| anyhow::anyhow!("Failed to process IPv6 Country CSV: {}", e))?;
                let bin_path = config.output_dir.join(ndb_ipv6_country::BIN_NAME);
                save_bin(db.entries(), bin_path, config.dry_run)?;
                tracing::info!("IPv6 Country database updated successfully.");
            }
            ndb_oui::CSV_NAME => {
                // Process OUI CSV file
                tracing::info!("Processing OUI file: {}", entry.path().display());
                let file = File::open(entry.path())
                    .map_err(|e| anyhow::anyhow!("Failed to open OUI CSV file: {}", e))?;
                let db = ndb_oui::OuiDb::from_csv(file)
                    .map_err(|e| anyhow::anyhow!("Failed to process OUI CSV: {}", e))?;
                let bin_path = config.output_dir.join(ndb_oui::BIN_NAME);
                save_bin(db.entries(), bin_path, config.dry_run)?;
            }
            ndb_tcp_service::CSV_NAME => {
                // Process TCP Service CSV file
                tracing::info!("Processing TCP Service file: {}", entry.path().display());
                let file = File::open(entry.path())
                    .map_err(|e| anyhow::anyhow!("Failed to open TCP Service CSV file: {}", e))?;
                let db = ndb_tcp_service::TcpServiceDb::from_csv(file)
                    .map_err(|e| anyhow::anyhow!("Failed to process TCP Service CSV: {}", e))?;
                let bin_path = config.output_dir.join(ndb_tcp_service::BIN_NAME);
                save_bin(db.entries(), bin_path, config.dry_run)?;
                tracing::info!("TCP Service database updated successfully.");
            }
            ndb_udp_service::CSV_NAME => {
                // Process UDP Service CSV file
                tracing::info!("Processing UDP Service file: {}", entry.path().display());
                let file = File::open(entry.path())
                    .map_err(|e| anyhow::anyhow!("Failed to open UDP Service CSV file: {}", e))?;
                let db = ndb_udp_service::UdpServiceDb::from_csv(file)
                    .map_err(|e| anyhow::anyhow!("Failed to process UDP Service CSV: {}", e))?;
                let bin_path = config.output_dir.join(ndb_udp_service::BIN_NAME);
                save_bin(db.entries(), bin_path, config.dry_run)?;
                tracing::info!("UDP Service database updated successfully.");
            }
            _ => {
                tracing::warn!("Skipping unknown file: {}", entry.path().display());
            }
        }
    }
    Ok(())
}

pub fn save_bin<T: serde::Serialize>(value: T, file_path: PathBuf, dry_run: bool) -> Result<()> {
    if dry_run {
        let size = bincode::serde::encode_to_vec(&value, bincode::config::standard())?.len();
        tracing::info!("[dry-run] Would serialize {} bytes to {}", size, file_path.display());
        return Ok(());
    }
    if file_path.exists() {
        fs::remove_file(&file_path)?;
    }
    let mut f: fs::File = fs::File::create(file_path.clone())?;
    match bincode::serde::encode_into_std_write(value, &mut f, bincode::config::standard()) {
        Ok(size) => {
            f.sync_all()?;
            let file_metadata = f.metadata()?;
            tracing::debug!("Serialized {} bytes to {}", size, file_path.display());
            tracing::debug!("File size: {} bytes", file_metadata.len());
            Ok(())
        },
        Err(e) => {
            Err(anyhow::anyhow!("Failed to serialize data: {}", e))
        },
    }
}
