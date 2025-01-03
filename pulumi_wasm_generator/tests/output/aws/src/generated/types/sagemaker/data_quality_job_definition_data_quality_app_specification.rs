#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataQualityJobDefinitionDataQualityAppSpecification {
    /// Sets the environment variables in the container that the monitoring job runs. A list of key value pairs.
    #[builder(into, default)]
    #[serde(rename = "environment")]
    pub r#environment: Box<Option<std::collections::HashMap<String, String>>>,
    /// The container image that the data quality monitoring job runs.
    #[builder(into)]
    #[serde(rename = "imageUri")]
    pub r#image_uri: Box<String>,
    /// An Amazon S3 URI to a script that is called after analysis has been performed. Applicable only for the built-in (first party) containers.
    #[builder(into, default)]
    #[serde(rename = "postAnalyticsProcessorSourceUri")]
    pub r#post_analytics_processor_source_uri: Box<Option<String>>,
    /// An Amazon S3 URI to a script that is called per row prior to running analysis. It can base64 decode the payload and convert it into a flatted json so that the built-in container can use the converted data. Applicable only for the built-in (first party) containers.
    #[builder(into, default)]
    #[serde(rename = "recordPreprocessorSourceUri")]
    pub r#record_preprocessor_source_uri: Box<Option<String>>,
}
