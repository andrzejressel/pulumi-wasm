#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkgroupConfiguration {
    /// Integer for the upper data usage limit (cutoff) for the amount of bytes a single query in a workgroup is allowed to scan. Must be at least `10485760`.
    #[builder(into, default)]
    #[serde(rename = "bytesScannedCutoffPerQuery")]
    pub r#bytes_scanned_cutoff_per_query: Box<Option<i32>>,
    /// Boolean whether the settings for the workgroup override client-side settings. For more information, see [Workgroup Settings Override Client-Side Settings](https://docs.aws.amazon.com/athena/latest/ug/workgroups-settings-override.html). Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enforceWorkgroupConfiguration")]
    pub r#enforce_workgroup_configuration: Box<Option<bool>>,
    /// Configuration block for the Athena Engine Versioning. For more information, see [Athena Engine Versioning](https://docs.aws.amazon.com/athena/latest/ug/engine-versions.html). See Engine Version below.
    #[builder(into, default)]
    #[serde(rename = "engineVersion")]
    pub r#engine_version: Box<Option<super::super::types::athena::WorkgroupConfigurationEngineVersion>>,
    /// Role used in a notebook session for accessing the user's resources.
    #[builder(into, default)]
    #[serde(rename = "executionRole")]
    pub r#execution_role: Box<Option<String>>,
    /// Boolean whether Amazon CloudWatch metrics are enabled for the workgroup. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "publishCloudwatchMetricsEnabled")]
    pub r#publish_cloudwatch_metrics_enabled: Box<Option<bool>>,
    /// If set to true , allows members assigned to a workgroup to reference Amazon S3 Requester Pays buckets in queries. If set to false , workgroup members cannot query data from Requester Pays buckets, and queries that retrieve data from Requester Pays buckets cause an error. The default is false . For more information about Requester Pays buckets, see [Requester Pays Buckets](https://docs.aws.amazon.com/AmazonS3/latest/dev/RequesterPaysBuckets.html) in the Amazon Simple Storage Service Developer Guide.
    #[builder(into, default)]
    #[serde(rename = "requesterPaysEnabled")]
    pub r#requester_pays_enabled: Box<Option<bool>>,
    /// Configuration block with result settings. See Result Configuration below.
    #[builder(into, default)]
    #[serde(rename = "resultConfiguration")]
    pub r#result_configuration: Box<Option<super::super::types::athena::WorkgroupConfigurationResultConfiguration>>,
}
