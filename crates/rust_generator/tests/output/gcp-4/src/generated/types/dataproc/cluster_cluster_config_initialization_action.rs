#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterClusterConfigInitializationAction {
    /// The script to be executed during initialization of the cluster.
    /// The script must be a GCS file with a gs:// prefix.
    #[builder(into)]
    #[serde(rename = "script")]
    pub r#script: Box<String>,
    /// The maximum duration (in seconds) which `script` is
    /// allowed to take to execute its action. GCP will default to a predetermined
    /// computed value if not set (currently 300).
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "timeoutSec")]
    pub r#timeout_sec: Box<Option<i32>>,
}
