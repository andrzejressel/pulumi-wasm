#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IoTHubSharedAccessPolicy {
    /// The name of the shared access policy.
    #[builder(into, default)]
    #[serde(rename = "keyName")]
    pub r#key_name: Box<Option<String>>,
    /// The permissions assigned to the shared access policy.
    #[builder(into, default)]
    #[serde(rename = "permissions")]
    pub r#permissions: Box<Option<String>>,
    /// The primary key.
    #[builder(into, default)]
    #[serde(rename = "primaryKey")]
    pub r#primary_key: Box<Option<String>>,
    /// The secondary key.
    #[builder(into, default)]
    #[serde(rename = "secondaryKey")]
    pub r#secondary_key: Box<Option<String>>,
}