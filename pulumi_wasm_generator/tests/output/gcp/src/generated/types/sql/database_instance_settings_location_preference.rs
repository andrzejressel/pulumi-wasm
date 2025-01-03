#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DatabaseInstanceSettingsLocationPreference {
    /// A GAE application whose zone to remain
    /// in. Must be in the same region as this instance.
    #[builder(into, default)]
    #[serde(rename = "followGaeApplication")]
    pub r#follow_gae_application: Box<Option<String>>,
    /// The preferred Compute Engine zone for the secondary/failover.
    #[builder(into, default)]
    #[serde(rename = "secondaryZone")]
    pub r#secondary_zone: Box<Option<String>>,
    /// The preferred compute engine
    /// [zone](https://cloud.google.com/compute/docs/zones?hl=en).
    #[builder(into, default)]
    #[serde(rename = "zone")]
    pub r#zone: Box<Option<String>>,
}
