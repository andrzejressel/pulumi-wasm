#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IntegrationAccountAgreementGuestIdentity {
    /// The authenticating body that provides unique guest identities to organizations.
    #[builder(into)]
    #[serde(rename = "qualifier")]
    pub r#qualifier: Box<String>,
    /// The value that identifies the documents that your logic apps receive.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
