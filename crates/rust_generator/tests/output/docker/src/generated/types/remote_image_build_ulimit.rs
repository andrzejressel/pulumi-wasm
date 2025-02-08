#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RemoteImageBuildUlimit {
    /// soft limit
    #[builder(into)]
    #[serde(rename = "hard")]
    pub r#hard: Box<i32>,
    /// type of ulimit, e.g. `nofile`
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// hard limit
    #[builder(into)]
    #[serde(rename = "soft")]
    pub r#soft: Box<i32>,
}
