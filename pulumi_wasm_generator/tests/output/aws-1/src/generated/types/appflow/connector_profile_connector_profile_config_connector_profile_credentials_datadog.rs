#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfileCredentialsDatadog {
    #[builder(into)]
    #[serde(rename = "apiKey")]
    pub r#api_key: Box<String>,
    /// Application keys, in conjunction with your API key, give you full access to Datadog’s programmatic API. Application keys are associated with the user account that created them. The application key is used to log all requests made to the API.
    #[builder(into)]
    #[serde(rename = "applicationKey")]
    pub r#application_key: Box<String>,
}
