#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceConfigReplica {
    /// If true, this location is designated as the default leader location where
    /// leader replicas are placed.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "defaultLeaderLocation")]
    pub r#default_leader_location: Box<Option<bool>>,
    /// The location of the serving resources, e.g. "us-central1".
    #[builder(into, default)]
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// Indicates the type of replica.  See the [replica types
    /// documentation](https://cloud.google.com/spanner/docs/replication#replica_types)
    /// for more details.
    /// Possible values are: `READ_WRITE`, `READ_ONLY`, `WITNESS`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
