#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetVirtualHubConnectionRoutingPropagatedRouteTable {
    /// The list of labels assigned to this route table.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Vec<String>>,
    /// A list of Route Table IDs associated with this Virtual Hub Connection.
    #[builder(into)]
    #[serde(rename = "routeTableIds")]
    pub r#route_table_ids: Box<Vec<String>>,
}
