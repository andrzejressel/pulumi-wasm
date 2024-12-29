#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetApplicationProvidersApplicationProvider {
    /// ARN of the application provider.
    #[builder(into)]
    #[serde(rename = "applicationProviderArn")]
    pub r#application_provider_arn: Box<String>,
    /// An object describing how IAM Identity Center represents the application provider in the portal. See `display_data` below.
    #[builder(into, default)]
    #[serde(rename = "displayDatas")]
    pub r#display_datas: Box<Option<Vec<super::super::types::ssoadmin::GetApplicationProvidersApplicationProviderDisplayData>>>,
    /// Protocol that the application provider uses to perform federation. Valid values are `SAML` and `OAUTH`.
    #[builder(into)]
    #[serde(rename = "federationProtocol")]
    pub r#federation_protocol: Box<String>,
}
