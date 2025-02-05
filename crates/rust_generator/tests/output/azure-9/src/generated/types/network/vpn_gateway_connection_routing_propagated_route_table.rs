#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VpnGatewayConnectionRoutingPropagatedRouteTable {
    /// A list of labels to assign to this route table.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<Vec<String>>>,
    /// A list of Route Table IDs to associated with this VPN Gateway Connection.
    #[builder(into)]
    #[serde(rename = "routeTableIds")]
    pub r#route_table_ids: Box<Vec<String>>,
}
