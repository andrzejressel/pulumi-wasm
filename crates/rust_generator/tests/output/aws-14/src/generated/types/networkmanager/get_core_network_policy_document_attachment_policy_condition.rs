#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCoreNetworkPolicyDocumentAttachmentPolicyCondition {
    /// string value
    #[builder(into, default)]
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    /// Valid values include: `equals`, `not-equals`, `contains`, `begins-with`.
    #[builder(into, default)]
    #[serde(rename = "operator")]
    pub r#operator: Box<Option<String>>,
    /// Valid values include: `account-id`, `any`, `tag-value`, `tag-exists`, `resource-id`, `region`, `attachment-type`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// string value
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
