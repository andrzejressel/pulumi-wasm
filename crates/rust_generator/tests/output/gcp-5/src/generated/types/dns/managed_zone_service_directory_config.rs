#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ManagedZoneServiceDirectoryConfig {
    /// The namespace associated with the zone.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "namespace")]
    pub r#namespace: Box<super::super::types::dns::ManagedZoneServiceDirectoryConfigNamespace>,
}
