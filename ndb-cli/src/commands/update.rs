use crate::commands::AppConfig;
use anyhow::Result;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

/// Update all binary databases from CSV sources in the configured input directory.
///
/// Each CSV file is converted into a compact binary representation
/// and written into the configured output directory. Unknown files are skipped.
pub fn update_bin_db(config: AppConfig) -> Result<()> {
    for entry in config.input_dir.read_dir()? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap_or_default();

        match file_name {
            ndb_oui::CSV_NAME => {
                process_csv_file(
                    entry.path().as_path(),
                    config.output_dir.as_path(),
                    ndb_oui::BIN_NAME,
                    config.dry_run,
                    "OUI",
                    |file| {
                        let db = ndb_oui::OuiDb::from_csv(file)?;
                        Ok(db.entries())
                    },
                )?;
            }
            ndb_tcp_service::CSV_NAME => {
                process_csv_file(
                    entry.path().as_path(),
                    config.output_dir.as_path(),
                    ndb_tcp_service::BIN_NAME,
                    config.dry_run,
                    "TCP Service",
                    |file| {
                        let db = ndb_tcp_service::TcpServiceDb::from_csv(file)?;
                        Ok(db.entries())
                    },
                )?;
            }
            ndb_udp_service::CSV_NAME => {
                process_csv_file(
                    entry.path().as_path(),
                    config.output_dir.as_path(),
                    ndb_udp_service::BIN_NAME,
                    config.dry_run,
                    "UDP Service",
                    |file| {
                        let db = ndb_udp_service::UdpServiceDb::from_csv(file)?;
                        Ok(db.entries())
                    },
                )?;
            }
            _ => {
                tracing::warn!("Skipping unknown file: {}", entry.path().display());
            }
        }
    }

    Ok(())
}

/// Generic helper to process a single CSV-based dataset.
fn process_csv_file<T, F>(
    entry_path: &Path,
    output_dir: &Path,
    bin_name: &str,
    dry_run: bool,
    label: &str,
    build_entries: F,
) -> Result<()>
where
    T: serde::Serialize,
    F: FnOnce(File) -> Result<T>,
{
    tracing::info!("Processing {} file: {}", label, entry_path.display());

    let file = File::open(entry_path)
        .map_err(|e| anyhow::anyhow!("Failed to open {} CSV file: {}", label, e))?;

    let entries = build_entries(file)
        .map_err(|e| anyhow::anyhow!("Failed to process {} CSV: {}", label, e))?;

    let bin_path = output_dir.join(bin_name);
    save_bin(entries, bin_path, dry_run)?;

    tracing::info!("{} database updated successfully.", label);
    Ok(())
}

/// Serialize a value into a bincode binary file.
///
/// When `dry_run` is enabled, this only reports the number of bytes that
/// would be written without touching the filesystem.
pub fn save_bin<T: serde::Serialize>(value: T, file_path: PathBuf, dry_run: bool) -> Result<()> {
    if dry_run {
        let size = bincode::serde::encode_to_vec(&value, bincode::config::standard())?.len();
        tracing::info!(
            "[dry-run] Would serialize {} bytes to {}",
            size,
            file_path.display()
        );
        return Ok(());
    }

    if file_path.exists() {
        fs::remove_file(&file_path)?;
    }

    let mut file = File::create(&file_path)?;
    match bincode::serde::encode_into_std_write(value, &mut file, bincode::config::standard()) {
        Ok(size) => {
            file.sync_all()?;
            let file_metadata = file.metadata()?;
            tracing::debug!("Serialized {} bytes to {}", size, file_path.display());
            tracing::debug!("File size: {} bytes", file_metadata.len());
            Ok(())
        }
        Err(e) => Err(anyhow::anyhow!("Failed to serialize data: {}", e)),
    }
}
