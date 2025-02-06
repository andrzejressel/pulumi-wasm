#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsDynatrace {
    /// The API tokens used by Dynatrace API to authenticate various API calls.
    #[builder(into)]
    #[serde(rename = "apiToken")]
    pub r#api_token: Box<String>,
}
