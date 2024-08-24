#[derive(serde::Serialize)]
pub struct HyperdriveConfigCaching {
    /// Disable caching for this Hyperdrive configuration.
    #[serde(rename = "disabled")]
    pub r#disabled: Box<Option<bool>>,
}
