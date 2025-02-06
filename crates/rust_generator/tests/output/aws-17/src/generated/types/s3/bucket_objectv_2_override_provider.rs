#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketObjectv2OverrideProvider {
    /// Override the provider `default_tags` configuration block.
    #[builder(into, default)]
    #[serde(rename = "defaultTags")]
    pub r#default_tags: Box<Option<super::super::types::s3::BucketObjectv2OverrideProviderDefaultTags>>,
}
