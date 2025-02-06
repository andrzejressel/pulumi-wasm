#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFhirServiceAuthentication {
    /// The intended audience to receive authentication tokens for the service. The default value is `https://<name>.fhir.azurehealthcareapis.com`.
    #[builder(into)]
    #[serde(rename = "audience")]
    pub r#audience: Box<String>,
    #[builder(into)]
    #[serde(rename = "authority")]
    pub r#authority: Box<String>,
    #[builder(into)]
    #[serde(rename = "smartProxyEnabled")]
    pub r#smart_proxy_enabled: Box<bool>,
}
