#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StorageLensConfigurationStorageLensConfigurationExclude {
    /// List of S3 bucket ARNs.
    #[builder(into, default)]
    #[serde(rename = "buckets")]
    pub r#buckets: Box<Option<Vec<String>>>,
    /// List of AWS Regions.
    #[builder(into, default)]
    #[serde(rename = "regions")]
    pub r#regions: Box<Option<Vec<String>>>,
}
