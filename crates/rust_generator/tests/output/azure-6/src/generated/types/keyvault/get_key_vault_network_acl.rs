#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetKeyVaultNetworkAcl {
    #[builder(into)]
    #[serde(rename = "bypass")]
    pub r#bypass: Box<String>,
    #[builder(into)]
    #[serde(rename = "defaultAction")]
    pub r#default_action: Box<String>,
    #[builder(into)]
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "virtualNetworkSubnetIds")]
    pub r#virtual_network_subnet_ids: Box<Vec<String>>,
}
