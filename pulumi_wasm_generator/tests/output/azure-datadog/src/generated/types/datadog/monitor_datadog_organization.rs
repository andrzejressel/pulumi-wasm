#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MonitorDatadogOrganization {
    /// Api key associated to the Datadog organization. Changing this forces a new Datadog Monitor to be created.
    #[builder(into)]
    #[serde(rename = "apiKey")]
    pub r#api_key: Box<String>,
    /// Application key associated to the Datadog organization. Changing this forces a new Datadog Monitor to be created.
    #[builder(into)]
    #[serde(rename = "applicationKey")]
    pub r#application_key: Box<String>,
    /// The ID of the enterprise_app. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "enterpriseAppId")]
    pub r#enterprise_app_id: Box<Option<String>>,
    /// The ID of the Datadog Monitor.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The auth code used to linking to an existing Datadog organization. Changing this forces a new Datadog Monitor to be created.
    #[builder(into, default)]
    #[serde(rename = "linkingAuthCode")]
    pub r#linking_auth_code: Box<Option<String>>,
    /// The ID of the linking_client. Changing this forces a new Datadog Monitor to be created.
    #[builder(into, default)]
    #[serde(rename = "linkingClientId")]
    pub r#linking_client_id: Box<Option<String>>,
    /// The name of the user that will be associated with the Datadog Monitor. Changing this forces a new Datadog Monitor to be created.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The redirect uri for linking. Changing this forces a new Datadog Monitor to be created.
    #[builder(into, default)]
    #[serde(rename = "redirectUri")]
    pub r#redirect_uri: Box<Option<String>>,
}
