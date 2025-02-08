#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileProperties {
    /// The connector-specific credentials required when using Amplitude. See Amplitude Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "amplitude")]
    pub r#amplitude: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesAmplitude>>,
    /// The connector-specific profile properties required when using the custom connector. See Custom Connector Profile Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "customConnector")]
    pub r#custom_connector: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesCustomConnector>>,
    /// Connector-specific properties required when using Datadog. See Generic Connector Profile Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "datadog")]
    pub r#datadog: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesDatadog>>,
    /// The connector-specific properties required when using Dynatrace. See Generic Connector Profile Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "dynatrace")]
    pub r#dynatrace: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesDynatrace>>,
    /// The connector-specific credentials required when using Google Analytics. See Google Analytics Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "googleAnalytics")]
    pub r#google_analytics: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesGoogleAnalytics>>,
    /// The connector-specific credentials required when using Amazon Honeycode. See Honeycode Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "honeycode")]
    pub r#honeycode: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesHoneycode>>,
    /// The connector-specific properties required when using Infor Nexus. See Generic Connector Profile Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "inforNexus")]
    pub r#infor_nexus: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesInforNexus>>,
    /// Connector-specific properties required when using Marketo. See Generic Connector Profile Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "marketo")]
    pub r#marketo: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesMarketo>>,
    /// Connector-specific properties required when using Amazon Redshift. See Redshift Connector Profile Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "redshift")]
    pub r#redshift: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesRedshift>>,
    /// The connector-specific properties required when using Salesforce. See Salesforce Connector Profile Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "salesforce")]
    pub r#salesforce: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSalesforce>>,
    /// The connector-specific properties required when using SAPOData. See SAPOData Connector Profile Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "sapoData")]
    pub r#sapo_data: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSapoData>>,
    /// The connector-specific properties required when using ServiceNow. See Generic Connector Profile Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "serviceNow")]
    pub r#service_now: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesServiceNow>>,
    /// Connector-specific credentials required when using Singular. See Singular Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "singular")]
    pub r#singular: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSingular>>,
    /// Connector-specific properties required when using Slack. See Generic Connector Profile Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "slack")]
    pub r#slack: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSlack>>,
    /// The connector-specific properties required when using Snowflake. See Snowflake Connector Profile Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "snowflake")]
    pub r#snowflake: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesSnowflake>>,
    /// The connector-specific credentials required when using Trend Micro. See Trend Micro Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "trendmicro")]
    pub r#trendmicro: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesTrendmicro>>,
    /// Connector-specific properties required when using Veeva. See Generic Connector Profile Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "veeva")]
    pub r#veeva: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesVeeva>>,
    /// Connector-specific properties required when using Zendesk. See Generic Connector Profile Properties for more details.
    #[builder(into, default)]
    #[serde(rename = "zendesk")]
    pub r#zendesk: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesZendesk>>,
}
