#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetEbsVolumesFilter {
    /// Name of the field to filter by, as defined by
    /// [the underlying AWS API](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeVolumes.html).
    /// For example, if matching against the `size` filter, use:
    /// 
    /// ```ignore
    /// use pulumi_wasm_rust::Output;
    /// use pulumi_wasm_rust::{add_export, pulumi_main};
    /// #[pulumi_main]
    /// fn test_main() -> Result<(), Error> {
    ///     let tenOrTwentyGbVolumes = get_ebs_volumes::invoke(
    ///         GetEbsVolumesArgs::builder()
    ///             .filters(
    ///                 vec![
    ///                     GetEbsVolumesFilter::builder().name("size").values(vec!["10", "20",])
    ///                     .build_struct(),
    ///                 ],
    ///             )
    ///             .build_struct(),
    ///     );
    /// }
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