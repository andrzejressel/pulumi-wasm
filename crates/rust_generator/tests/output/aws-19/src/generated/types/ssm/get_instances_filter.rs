#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetInstancesFilter {
    /// Name of the filter field. Valid values can be found in the [SSM InstanceInformationStringFilter API Reference](https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_InstanceInformationStringFilter.html).
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Set of values that are accepted for the given filter field. Results will be selected if any given value matches.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
