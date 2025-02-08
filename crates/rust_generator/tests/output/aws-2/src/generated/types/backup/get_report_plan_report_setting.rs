#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetReportPlanReportSetting {
    /// (Optional) Specifies the list of accounts a report covers.
    #[builder(into)]
    #[serde(rename = "accounts")]
    pub r#accounts: Box<Vec<String>>,
    /// ARNs of the frameworks a report covers.
    #[builder(into)]
    #[serde(rename = "frameworkArns")]
    pub r#framework_arns: Box<Vec<String>>,
    /// Specifies the number of frameworks a report covers.
    #[builder(into)]
    #[serde(rename = "numberOfFrameworks")]
    pub r#number_of_frameworks: Box<i32>,
    /// (Optional) Specifies the list of Organizational Units a report covers.
    #[builder(into)]
    #[serde(rename = "organizationUnits")]
    pub r#organization_units: Box<Vec<String>>,
    /// (Optional) Specifies the list of regions a report covers.
    #[builder(into)]
    #[serde(rename = "regions")]
    pub r#regions: Box<Vec<String>>,
    /// Identifies the report template for the report. Reports are built using a report template.
    #[builder(into)]
    #[serde(rename = "reportTemplate")]
    pub r#report_template: Box<String>,
}
