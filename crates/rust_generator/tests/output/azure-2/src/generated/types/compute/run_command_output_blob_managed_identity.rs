#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RunCommandOutputBlobManagedIdentity {
    /// The client ID of the managed identity.
    #[builder(into, default)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<Option<String>>,
    /// The object ID of the managed identity.
    #[builder(into, default)]
    #[serde(rename = "objectId")]
    pub r#object_id: Box<Option<String>>,
}
