#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FunctionEventTrigger {
    /// The type of event to observe. For example: `"google.storage.object.finalize"`.
    /// See the documentation on [calling Cloud Functions](https://cloud.google.com/functions/docs/calling/) for a
    /// full reference of accepted triggers.
    #[builder(into)]
    #[serde(rename = "eventType")]
    pub r#event_type: Box<String>,
    /// Specifies policy for failed executions. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "failurePolicy")]
    pub r#failure_policy: Box<Option<super::super::types::cloudfunctions::FunctionEventTriggerFailurePolicy>>,
    /// Required. The name or partial URI of the resource from
    /// which to observe events. For example, `"myBucket"` or `"projects/my-project/topics/my-topic"`
    #[builder(into)]
    #[serde(rename = "resource")]
    pub r#resource: Box<String>,
}
