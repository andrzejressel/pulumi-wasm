#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCostCategoryRuleInheritedValue {
    /// Key to extract cost category values.
    #[builder(into)]
    #[serde(rename = "dimensionKey")]
    pub r#dimension_key: Box<String>,
    /// Name of the dimension that's used to group costs. If you specify `LINKED_ACCOUNT_NAME`, the cost category value is based on account name. If you specify `TAG`, the cost category value will be based on the value of the specified tag key. Valid values are `LINKED_ACCOUNT_NAME`, `TAG`
    #[builder(into)]
    #[serde(rename = "dimensionName")]
    pub r#dimension_name: Box<String>,
}
