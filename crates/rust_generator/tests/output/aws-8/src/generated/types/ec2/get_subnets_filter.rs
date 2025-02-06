#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSubnetsFilter {
    /// Name of the field to filter by, as defined by
    /// [the underlying AWS API](http://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeSubnets.html).
    /// For example, if matching against tag `Name`, use:
    /// 
    /// ```yaml
    /// variables:
    ///   selected:
    ///     fn::invoke:
    ///       function: aws:ec2:getSubnets
    ///       arguments:
    ///         filters:
    ///           - name: tag:Name
    ///             values:
    ///               - ""
    /// ```
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Set of values that are accepted for the given field.
    /// Subnet IDs will be selected if any one of the given values match.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
