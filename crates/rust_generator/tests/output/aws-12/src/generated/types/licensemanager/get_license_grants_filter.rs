#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetLicenseGrantsFilter {
    /// Name of the field to filter by, as defined by
    /// [the underlying AWS API](https://docs.aws.amazon.com/license-manager/latest/APIReference/API_ListReceivedGrants.html#API_ListReceivedGrants_RequestSyntax).
    /// For example, if filtering using `ProductSKU`, use:
    /// 
    /// ```yaml
    /// variables:
    ///   selected:
    ///     fn::invoke:
    ///       function: aws:licensemanager:getLicenseGrants
    ///       arguments:
    ///         filters:
    ///           - name: ProductSKU
    ///             values:
    ///               - ""
    /// ```
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Set of values that are accepted for the given field.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
