#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkforcePoolProviderOidcClientSecret {
    /// The value of the client secret.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<super::super::types::iam::WorkforcePoolProviderOidcClientSecretValue>>,
}
