#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EndpointElasticsearchSettings {
    /// Endpoint for the OpenSearch cluster.
    #[builder(into)]
    #[serde(rename = "endpointUri")]
    pub r#endpoint_uri: Box<String>,
    /// Maximum number of seconds for which DMS retries failed API requests to the OpenSearch cluster. Default is `300`.
    #[builder(into, default)]
    #[serde(rename = "errorRetryDuration")]
    pub r#error_retry_duration: Box<Option<i32>>,
    /// Maximum percentage of records that can fail to be written before a full load operation stops. Default is `10`.
    #[builder(into, default)]
    #[serde(rename = "fullLoadErrorPercentage")]
    pub r#full_load_error_percentage: Box<Option<i32>>,
    /// ARN of the IAM Role with permissions to write to the OpenSearch cluster.
    #[builder(into)]
    #[serde(rename = "serviceAccessRoleArn")]
    pub r#service_access_role_arn: Box<String>,
    /// Enable to migrate documentation using the documentation type `_doc`. OpenSearch and an Elasticsearch clusters only support the _doc documentation type in versions 7.x and later. The default value is `false`.
    #[builder(into, default)]
    #[serde(rename = "useNewMappingType")]
    pub r#use_new_mapping_type: Box<Option<bool>>,
}
