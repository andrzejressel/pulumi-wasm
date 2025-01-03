#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileCredentials {
    /// The connector-specific credentials required when using Amplitude. See Amplitude Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "amplitude")]
    pub r#amplitude: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsAmplitude>>,
    /// The connector-specific profile credentials required when using the custom connector. See Custom Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "customConnector")]
    pub r#custom_connector: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnector>>,
    /// Connector-specific credentials required when using Datadog. See Datadog Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "datadog")]
    pub r#datadog: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsDatadog>>,
    /// The connector-specific credentials required when using Dynatrace. See Dynatrace Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "dynatrace")]
    pub r#dynatrace: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsDynatrace>>,
    /// The connector-specific credentials required when using Google Analytics. See Google Analytics Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "googleAnalytics")]
    pub r#google_analytics: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsGoogleAnalytics>>,
    /// The connector-specific credentials required when using Amazon Honeycode. See Honeycode Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "honeycode")]
    pub r#honeycode: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsHoneycode>>,
    /// The connector-specific credentials required when using Infor Nexus. See Infor Nexus Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "inforNexus")]
    pub r#infor_nexus: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsInforNexus>>,
    /// Connector-specific credentials required when using Marketo. See Marketo Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "marketo")]
    pub r#marketo: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsMarketo>>,
    /// Connector-specific credentials required when using Amazon Redshift. See Redshift Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "redshift")]
    pub r#redshift: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsRedshift>>,
    /// The connector-specific credentials required when using Salesforce. See Salesforce Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "salesforce")]
    pub r#salesforce: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSalesforce>>,
    /// The connector-specific credentials required when using SAPOData. See SAPOData Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "sapoData")]
    pub r#sapo_data: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSapoData>>,
    /// The connector-specific credentials required when using ServiceNow. See ServiceNow Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "serviceNow")]
    pub r#service_now: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsServiceNow>>,
    /// Connector-specific credentials required when using Singular. See Singular Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "singular")]
    pub r#singular: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSingular>>,
    /// Connector-specific credentials required when using Slack. See Slack Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "slack")]
    pub r#slack: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSlack>>,
    /// The connector-specific credentials required when using Snowflake. See Snowflake Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "snowflake")]
    pub r#snowflake: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsSnowflake>>,
    /// The connector-specific credentials required when using Trend Micro. See Trend Micro Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "trendmicro")]
    pub r#trendmicro: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsTrendmicro>>,
    /// Connector-specific credentials required when using Veeva. See Veeva Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "veeva")]
    pub r#veeva: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsVeeva>>,
    /// Connector-specific credentials required when using Zendesk. See Zendesk Connector Profile Credentials for more details.
    #[builder(into, default)]
    #[serde(rename = "zendesk")]
    pub r#zendesk: Box<Option<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsZendesk>>,
}
