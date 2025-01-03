#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkforcePoolProviderExtraAttributesOauth2ClientClientSecretValue {
    /// The plain text of the client secret value.
    #[builder(into)]
    #[serde(rename = "plainText")]
    pub r#plain_text: Box<String>,
    /// (Output)
    /// A thumbprint to represent the current client secret value.
    #[builder(into, default)]
    #[serde(rename = "thumbprint")]
    pub r#thumbprint: Box<Option<String>>,
}
