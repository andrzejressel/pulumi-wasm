#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRuntimeVersionsRuntimeVersion {
    /// Date of deprecation if the runtme version is deprecated.
    #[builder(into)]
    #[serde(rename = "deprecationDate")]
    pub r#deprecation_date: Box<String>,
    /// Description of the runtime version, created by Amazon.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// Date that the runtime version was released.
    #[builder(into)]
    #[serde(rename = "releaseDate")]
    pub r#release_date: Box<String>,
    /// Name of the runtime version.
    /// For a list of valid runtime versions, see [Canary Runtime Versions](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Synthetics_Canaries_Library.html).
    #[builder(into)]
    #[serde(rename = "versionName")]
    pub r#version_name: Box<String>,
}
