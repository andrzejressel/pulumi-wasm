#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetVpcsFilter {
    /// Name of the field to filter by, as defined by
    /// [the underlying AWS API](http://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeVpcs.html).
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Set of values that are accepted for the given field.
    /// A VPC will be selected if any one of the given values matches.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
