#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KafkaClusterRestProxy {
    /// The Azure Active Directory Security Group ID. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "securityGroupId")]
    pub r#security_group_id: Box<String>,
    /// The Azure Active Directory Security Group name. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** The `security_group_name` property will be Required in version 3.0 of the AzureRM Provider.
    #[builder(into)]
    #[serde(rename = "securityGroupName")]
    pub r#security_group_name: Box<String>,
}
