#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceScaling {
    /// Minimum number of instances for the service, to be divided among all revisions receiving traffic.
    #[builder(into, default)]
    #[serde(rename = "minInstanceCount")]
    pub r#min_instance_count: Box<Option<i32>>,
}
