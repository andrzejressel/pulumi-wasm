#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AlertRuleScheduledEventGrouping {
    /// The aggregation type of grouping the events. Possible values are `AlertPerResult` and `SingleAlert`.
    #[builder(into)]
    #[serde(rename = "aggregationMethod")]
    pub r#aggregation_method: Box<String>,
}
