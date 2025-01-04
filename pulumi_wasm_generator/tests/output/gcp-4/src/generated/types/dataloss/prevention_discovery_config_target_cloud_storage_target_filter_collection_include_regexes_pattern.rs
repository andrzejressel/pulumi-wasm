#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDiscoveryConfigTargetCloudStorageTargetFilterCollectionIncludeRegexesPattern {
    /// Regex for Cloud Storage.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "cloudStorageRegex")]
    pub r#cloud_storage_regex: Box<Option<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudStorageTargetFilterCollectionIncludeRegexesPatternCloudStorageRegex>>,
}
