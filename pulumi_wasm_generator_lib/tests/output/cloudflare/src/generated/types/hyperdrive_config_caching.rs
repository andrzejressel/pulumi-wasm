#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct HyperdriveConfigCaching {
    /// Disable caching for this Hyperdrive configuration.
    #[builder(into, default)]
    #[serde(rename = "disabled")]
    pub r#disabled: Box<Option<bool>>,
    /// Configure the `max_age` value of this Hyperdrive configuration.
    #[builder(into, default)]
    #[serde(rename = "maxAge")]
    pub r#max_age: Box<Option<i32>>,
    /// Disable caching for this Hyperdrive configuration.
    #[builder(into, default)]
    #[serde(rename = "staleWhileRevalidate")]
    pub r#stale_while_revalidate: Box<Option<i32>>,
}
