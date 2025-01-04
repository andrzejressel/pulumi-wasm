#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketServerSideEncryptionConfiguration {
    /// A single object for server-side encryption by default configuration. (documented below)
    #[builder(into)]
    #[serde(rename = "rule")]
    pub r#rule: Box<super::super::types::s3::BucketServerSideEncryptionConfigurationRule>,
}
