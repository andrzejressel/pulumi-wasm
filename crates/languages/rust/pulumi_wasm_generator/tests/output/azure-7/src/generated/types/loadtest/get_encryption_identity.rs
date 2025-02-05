#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetEncryptionIdentity {
    /// The User Assigned Identity ID that is assigned to this Load Test Encryption.
    #[builder(into)]
    #[serde(rename = "identityId")]
    pub r#identity_id: Box<String>,
    /// Type of Managed Service Identity that is assigned to this Load Test Encryption.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
