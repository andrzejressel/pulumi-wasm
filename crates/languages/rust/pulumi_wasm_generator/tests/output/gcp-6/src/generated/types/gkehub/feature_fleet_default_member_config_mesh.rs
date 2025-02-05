#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FeatureFleetDefaultMemberConfigMesh {
    /// Whether to automatically manage Service Mesh
    /// Possible values are: `MANAGEMENT_UNSPECIFIED`, `MANAGEMENT_AUTOMATIC`, `MANAGEMENT_MANUAL`.
    #[builder(into)]
    #[serde(rename = "management")]
    pub r#management: Box<String>,
}
