#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PrivateCloudCircuit {
    /// The ID of the ExpressRoute Circuit.
    #[builder(into, default)]
    #[serde(rename = "expressRouteId")]
    pub r#express_route_id: Box<Option<String>>,
    /// The ID of the ExpressRoute Circuit private peering.
    #[builder(into, default)]
    #[serde(rename = "expressRoutePrivatePeeringId")]
    pub r#express_route_private_peering_id: Box<Option<String>>,
    /// The CIDR of the primary subnet.
    #[builder(into, default)]
    #[serde(rename = "primarySubnetCidr")]
    pub r#primary_subnet_cidr: Box<Option<String>>,
    /// The CIDR of the secondary subnet.
    #[builder(into, default)]
    #[serde(rename = "secondarySubnetCidr")]
    pub r#secondary_subnet_cidr: Box<Option<String>>,
}