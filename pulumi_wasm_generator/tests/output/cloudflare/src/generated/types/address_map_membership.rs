#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AddressMapMembership {
    /// Controls whether the membership can be deleted via the API or not.
    #[builder(into, default)]
    #[serde(rename = "canDelete")]
    pub r#can_delete: Box<Option<bool>>,
    /// Identifier of the account or zone.
    #[builder(into)]
    #[serde(rename = "identifier")]
    pub r#identifier: Box<String>,
    /// The type of the membership.
    #[builder(into)]
    #[serde(rename = "kind")]
    pub r#kind: Box<String>,
}
