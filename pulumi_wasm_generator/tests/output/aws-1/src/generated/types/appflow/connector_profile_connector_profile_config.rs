#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectorProfileConnectorProfileConfig {
    /// The connector-specific credentials required by each connector. See Connector Profile Credentials for more details.
    #[builder(into)]
    #[serde(rename = "connectorProfileCredentials")]
    pub r#connector_profile_credentials: Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileCredentials>,
    /// The connector-specific properties of the profile configuration. See Connector Profile Properties for more details.
    #[builder(into)]
    #[serde(rename = "connectorProfileProperties")]
    pub r#connector_profile_properties: Box<super::super::types::appflow::ConnectorProfileConnectorProfileConfigConnectorProfileProperties>,
}
