#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualHubRouteTableRoute {
    /// A list of destination addresses for this route.
    #[builder(into)]
    #[serde(rename = "destinations")]
    pub r#destinations: Box<Vec<String>>,
    /// The type of destinations. Possible values are `CIDR`, `ResourceId` and `Service`.
    #[builder(into)]
    #[serde(rename = "destinationsType")]
    pub r#destinations_type: Box<String>,
    /// The name which should be used for this route.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The next hop's resource ID.
    #[builder(into)]
    #[serde(rename = "nextHop")]
    pub r#next_hop: Box<String>,
    /// The type of next hop. Currently the only possible value is `ResourceId`. Defaults to `ResourceId`.
    /// 
    /// > **Note:** The Routes can alternatively be created using the virtual_hub_route_table_route resource. Using both inline and external routes is not supported and may result in unexpected configuration.
    #[builder(into, default)]
    #[serde(rename = "nextHopType")]
    pub r#next_hop_type: Box<Option<String>>,
}
