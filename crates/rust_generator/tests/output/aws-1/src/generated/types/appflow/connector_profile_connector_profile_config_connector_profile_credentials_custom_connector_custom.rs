#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsCustomConnectorCustom {
    /// A map that holds custom authentication credentials.
    #[builder(into, default)]
    #[serde(rename = "credentialsMap")]
    pub r#credentials_map: Box<Option<std::collections::HashMap<String, String>>>,
    /// The custom authentication type that the connector uses.
    #[builder(into)]
    #[serde(rename = "customAuthenticationType")]
    pub r#custom_authentication_type: Box<String>,
}
