#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlowDestinationFlowConfig {
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
    /// This stores the information that is required to query a particular connector. See Destination Connector Properties for more information.
    #[builder(into)]
    #[serde(rename = "destinationConnectorProperties")]
    pub r#destination_connector_properties: Box<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorProperties>,
}
