#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkerScriptQueueBinding {
    /// The name of the global variable for the binding in your Worker code.
    #[builder(into)]
    #[serde(rename = "binding")]
    pub r#binding: Box<String>,
    /// Name of the queue you want to use.
    #[builder(into)]
    #[serde(rename = "queue")]
    pub r#queue: Box<String>,
}
