#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionProfileAlloydb {
    /// Required. The AlloyDB cluster ID that this connection profile is associated with.
    #[builder(into)]
    #[serde(rename = "clusterId")]
    pub r#cluster_id: Box<String>,
    /// Immutable. Metadata used to create the destination AlloyDB cluster.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "settings")]
    pub r#settings: Box<Option<super::super::types::databasemigrationservice::ConnectionProfileAlloydbSettings>>,
}
