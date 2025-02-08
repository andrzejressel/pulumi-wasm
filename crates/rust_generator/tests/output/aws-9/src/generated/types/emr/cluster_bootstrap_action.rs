#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterBootstrapAction {
    /// List of command line arguments to pass to the bootstrap action script.
    #[builder(into, default)]
    #[serde(rename = "args")]
    pub r#args: Box<Option<Vec<String>>>,
    /// Name of the bootstrap action.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Location of the script to run during a bootstrap action. Can be either a location in Amazon S3 or on a local file system.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
}
