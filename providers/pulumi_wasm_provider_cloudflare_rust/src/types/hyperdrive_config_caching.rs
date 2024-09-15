#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct HyperdriveConfigCaching {
    /// Disable caching for this Hyperdrive configuration.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "disabled")]
    pub r#disabled: Box<Option<bool>>,
}
