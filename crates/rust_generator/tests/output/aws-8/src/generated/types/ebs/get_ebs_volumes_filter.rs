#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetEbsVolumesFilter {
    /// Name of the field to filter by, as defined by
    /// [the underlying AWS API](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeVolumes.html).
    /// For example, if matching against the `size` filter, use:
    /// 
    /// ```yaml
    /// variables:
    ///   tenOrTwentyGbVolumes:
    ///     fn::invoke:
    ///       function: aws:ebs:getEbsVolumes
    ///       arguments:
    ///         filters:
    ///           - name: size
    ///             values:
    ///               - '10'
    ///               - '20'
    /// ```
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Set of values that are accepted for the given field.
    /// EBS Volume IDs will be selected if any one of the given values match.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
