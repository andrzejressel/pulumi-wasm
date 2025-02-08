#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SloBasicSliLatency {
    /// A duration string, e.g. 10s.
    /// Good service is defined to be the count of requests made to
    /// this service that return in no more than threshold.
    #[builder(into)]
    #[serde(rename = "threshold")]
    pub r#threshold: Box<String>,
}
