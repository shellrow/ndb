use serde::{Deserialize, Deserializer};

/// Deserialize a u8 value to a boolean
pub fn de_u8_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let v: u8 = Deserialize::deserialize(deserializer)?;
    Ok(v != 0)
}
