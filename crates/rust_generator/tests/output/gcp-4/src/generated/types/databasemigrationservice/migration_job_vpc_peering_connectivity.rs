#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MigrationJobVpcPeeringConnectivity {
    /// The name of the VPC network to peer with the Cloud SQL private network.
    #[builder(into, default)]
    #[serde(rename = "vpc")]
    pub r#vpc: Box<Option<String>>,
}
