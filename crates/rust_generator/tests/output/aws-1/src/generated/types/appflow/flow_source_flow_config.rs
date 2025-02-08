#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlowSourceFlowConfig {
    /// API version that the destination connector uses.
    #[builder(into, default)]
    #[serde(rename = "apiVersion")]
    pub r#api_version: Box<Option<String>>,
    /// Name of the connector profile. This name must be unique for each connector profile in the AWS account.
    #[builder(into, default)]
    #[serde(rename = "connectorProfileName")]
    pub r#connector_profile_name: Box<Option<String>>,
    /// Type of connector, such as Salesforce, Amplitude, and so on. Valid values are `Salesforce`, `Singular`, `Slack`, `Redshift`, `S3`, `Marketo`, `Googleanalytics`, `Zendesk`, `Servicenow`, `Datadog`, `Trendmicro`, `Snowflake`, `Dynatrace`, `Infornexus`, `Amplitude`, `Veeva`, `EventBridge`, `LookoutMetrics`, `Upsolver`, `Honeycode`, `CustomerProfiles`, `SAPOData`, and `CustomConnector`.
    #[builder(into)]
    #[serde(rename = "connectorType")]
    pub r#connector_type: Box<String>,
    /// Defines the configuration for a scheduled incremental data pull. If a valid configuration is provided, the fields specified in the configuration are used when querying for the incremental data pull. See Incremental Pull Config for more details.
    #[builder(into, default)]
    #[serde(rename = "incrementalPullConfig")]
    pub r#incremental_pull_config: Box<Option<super::super::types::appflow::FlowSourceFlowConfigIncrementalPullConfig>>,
    /// Information that is required to query a particular source connector. See Source Connector Properties for details.
    #[builder(into)]
    #[serde(rename = "sourceConnectorProperties")]
    pub r#source_connector_properties: Box<super::super::types::appflow::FlowSourceFlowConfigSourceConnectorProperties>,
}
