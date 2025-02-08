#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobTemplateConfigPubsubDestination {
    /// The name of the Pub/Sub topic to publish job completion notification to. For example: projects/{project}/topics/{topic}.
    #[builder(into, default)]
    #[serde(rename = "topic")]
    pub r#topic: Box<Option<String>>,
}
