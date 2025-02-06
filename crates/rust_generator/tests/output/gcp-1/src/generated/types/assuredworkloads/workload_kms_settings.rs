#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkloadKmsSettings {
    /// Required. Input only. Immutable. The time at which the Key Management Service will automatically create a new version of the crypto key and mark it as the primary.
    #[builder(into)]
    #[serde(rename = "nextRotationTime")]
    pub r#next_rotation_time: Box<String>,
    /// Required. Input only. Immutable. will be advanced by this period when the Key Management Service automatically rotates a key. Must be at least 24 hours and at most 876,000 hours.
    #[builder(into)]
    #[serde(rename = "rotationPeriod")]
    pub r#rotation_period: Box<String>,
}
