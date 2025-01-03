#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CustomLogSourceConfigurationProviderIdentity {
    /// The external ID used to estalish trust relationship with the AWS identity.
    #[builder(into)]
    #[serde(rename = "externalId")]
    pub r#external_id: Box<String>,
    /// The AWS identity principal.
    #[builder(into)]
    #[serde(rename = "principal")]
    pub r#principal: Box<String>,
}
