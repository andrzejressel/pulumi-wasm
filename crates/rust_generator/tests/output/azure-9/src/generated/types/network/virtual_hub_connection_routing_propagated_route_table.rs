#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualHubConnectionRoutingPropagatedRouteTable {
    /// The list of labels to assign to this route table.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<Vec<String>>>,
    /// A list of Route Table IDs to associated with this Virtual Hub Connection.
    #[builder(into, default)]
    #[serde(rename = "routeTableIds")]
    pub r#route_table_ids: Box<Option<Vec<String>>>,
}
