#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ThingGroupMetadataRootToParentGroup {
    #[builder(into, default)]
    #[serde(rename = "groupArn")]
    pub r#group_arn: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "groupName")]
    pub r#group_name: Box<Option<String>>,
}