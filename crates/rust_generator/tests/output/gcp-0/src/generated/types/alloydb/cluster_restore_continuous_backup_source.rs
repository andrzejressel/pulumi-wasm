#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterRestoreContinuousBackupSource {
    /// The name of the source cluster that this cluster is restored from.
    #[builder(into)]
    #[serde(rename = "cluster")]
    pub r#cluster: Box<String>,
    /// The point in time that this cluster is restored to, in RFC 3339 format.
    #[builder(into)]
    #[serde(rename = "pointInTime")]
    pub r#point_in_time: Box<String>,
}
