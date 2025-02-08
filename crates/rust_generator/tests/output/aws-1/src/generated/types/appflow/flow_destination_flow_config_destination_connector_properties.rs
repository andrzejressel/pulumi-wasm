#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FlowDestinationFlowConfigDestinationConnectorProperties {
    /// Properties that are required to query the custom Connector. See Custom Connector Destination Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "customConnector")]
    pub r#custom_connector: Box<Option<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesCustomConnector>>,
    /// Properties that are required to query Amazon Connect Customer Profiles. See Customer Profiles Destination Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "customerProfiles")]
    pub r#customer_profiles: Box<Option<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesCustomerProfiles>>,
    /// Properties that are required to query Amazon EventBridge. See Generic Destination Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "eventBridge")]
    pub r#event_bridge: Box<Option<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesEventBridge>>,
    /// Properties that are required to query Amazon Honeycode. See Generic Destination Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "honeycode")]
    pub r#honeycode: Box<Option<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesHoneycode>>,
    #[builder(into, default)]
    #[serde(rename = "lookoutMetrics")]
    pub r#lookout_metrics: Box<Option<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesLookoutMetrics>>,
    /// Properties that are required to query Marketo. See Generic Destination Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "marketo")]
    pub r#marketo: Box<Option<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesMarketo>>,
    /// Properties that are required to query Amazon Redshift. See Redshift Destination Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "redshift")]
    pub r#redshift: Box<Option<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesRedshift>>,
    /// Properties that are required to query Amazon S3. See S3 Destination Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "s3")]
    pub r#s_3: Box<Option<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesS3>>,
    /// Properties that are required to query Salesforce. See Salesforce Destination Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "salesforce")]
    pub r#salesforce: Box<Option<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesSalesforce>>,
    /// Properties that are required to query SAPOData. See SAPOData Destination Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "sapoData")]
    pub r#sapo_data: Box<Option<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesSapoData>>,
    /// Properties that are required to query Snowflake. See Snowflake Destination Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "snowflake")]
    pub r#snowflake: Box<Option<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesSnowflake>>,
    /// Properties that are required to query Upsolver. See Upsolver Destination Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "upsolver")]
    pub r#upsolver: Box<Option<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesUpsolver>>,
    /// Properties that are required to query Zendesk. See Zendesk Destination Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "zendesk")]
    pub r#zendesk: Box<Option<super::super::types::appflow::FlowDestinationFlowConfigDestinationConnectorPropertiesZendesk>>,
}
