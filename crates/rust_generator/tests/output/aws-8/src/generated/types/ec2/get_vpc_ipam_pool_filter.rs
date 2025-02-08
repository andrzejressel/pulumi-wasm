#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetVpcIpamPoolFilter {
    /// The name of the filter. Filter names are case-sensitive.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The filter values. Filter values are case-sensitive.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
