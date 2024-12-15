//! Type of product filter.

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
pub enum SupportedFilterTypes {
    /// Ship to country
    ShipToCountries,
    /// Double encryption status
    DoubleEncryptionStatus,
}
