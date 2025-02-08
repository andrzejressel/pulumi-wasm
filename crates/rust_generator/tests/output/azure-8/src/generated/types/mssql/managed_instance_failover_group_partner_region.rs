#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ManagedInstanceFailoverGroupPartnerRegion {
    /// The Azure Region where the Managed Instance Failover Group should exist. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// The partner replication role of the Managed Instance Failover Group.
    #[builder(into, default)]
    #[serde(rename = "role")]
    pub r#role: Box<Option<String>>,
}
