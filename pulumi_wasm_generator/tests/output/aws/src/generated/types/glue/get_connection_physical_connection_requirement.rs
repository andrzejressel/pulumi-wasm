#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetConnectionPhysicalConnectionRequirement {
    #[builder(into)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Box<String>,
    #[builder(into)]
    #[serde(rename = "securityGroupIdLists")]
    pub r#security_group_id_lists: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}