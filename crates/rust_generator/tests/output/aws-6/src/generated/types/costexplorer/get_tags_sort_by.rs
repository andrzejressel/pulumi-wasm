#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetTagsSortBy {
    /// key that's used to sort the data. Valid values are: `BlendedCost`,  `UnblendedCost`, `AmortizedCost`, `NetAmortizedCost`, `NetUnblendedCost`, `UsageQuantity`, `NormalizedUsageAmount`.
    #[builder(into, default)]
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    /// order that's used to sort the data. Valid values are: `ASCENDING`,  `DESCENDING`.
    #[builder(into, default)]
    #[serde(rename = "sortOrder")]
    pub r#sort_order: Box<Option<String>>,
}
