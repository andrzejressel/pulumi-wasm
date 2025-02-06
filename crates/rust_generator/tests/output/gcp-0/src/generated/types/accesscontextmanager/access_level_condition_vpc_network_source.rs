#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccessLevelConditionVpcNetworkSource {
    /// Sub networks within a VPC network.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "vpcSubnetwork")]
    pub r#vpc_subnetwork: Box<Option<super::super::types::accesscontextmanager::AccessLevelConditionVpcNetworkSourceVpcSubnetwork>>,
}
