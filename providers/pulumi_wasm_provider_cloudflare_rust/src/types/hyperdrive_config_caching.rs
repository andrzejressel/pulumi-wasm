#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct HyperdriveConfigCaching {
    /// Disable caching for this Hyperdrive configuration.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "disabled")]
    pub r#disabled: Box<Option<bool>>,
    /// Configure the `max_age` value of this Hyperdrive configuration.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "maxAge")]
    pub r#max_age: Box<Option<i32>>,
    /// Disable caching for this Hyperdrive configuration.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "staleWhileRevalidate")]
    pub r#stale_while_revalidate: Box<Option<i32>>,
}
