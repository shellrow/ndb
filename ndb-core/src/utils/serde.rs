use serde::{Deserialize, Deserializer};

pub fn de_u8_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let v: u8 = Deserialize::deserialize(deserializer)?;
    Ok(v != 0)
}
