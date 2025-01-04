#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketWebsite {
    /// Behaves as the bucket's directory index where
    /// missing objects are treated as potential directories.
    #[builder(into, default)]
    #[serde(rename = "mainPageSuffix")]
    pub r#main_page_suffix: Box<Option<String>>,
    /// The custom object to return when a requested
    /// resource is not found.
    #[builder(into, default)]
    #[serde(rename = "notFoundPage")]
    pub r#not_found_page: Box<Option<String>>,
}
