#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkConnectionMonitorEndpointFilter {
    /// A `item` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "items")]
    pub r#items: Box<Option<Vec<super::super::types::network::NetworkConnectionMonitorEndpointFilterItem>>>,
    /// The behaviour type of this endpoint filter. Currently the only allowed value is `Include`. Defaults to `Include`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
