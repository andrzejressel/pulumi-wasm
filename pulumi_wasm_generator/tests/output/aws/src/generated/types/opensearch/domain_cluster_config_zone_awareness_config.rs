#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainClusterConfigZoneAwarenessConfig {
    /// Number of Availability Zones for the domain to use with `zone_awareness_enabled`. Defaults to `2`. Valid values: `2` or `3`.
    #[builder(into, default)]
    #[serde(rename = "availabilityZoneCount")]
    pub r#availability_zone_count: Box<Option<i32>>,
}
