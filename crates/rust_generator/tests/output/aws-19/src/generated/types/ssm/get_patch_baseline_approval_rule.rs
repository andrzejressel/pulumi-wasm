#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetPatchBaselineApprovalRule {
    /// Number of days after the release date of each patch matched by the rule the patch is marked as approved in the patch baseline.
    #[builder(into)]
    #[serde(rename = "approveAfterDays")]
    pub r#approve_after_days: Box<i32>,
    /// Cutoff date for auto approval of released patches. Any patches released on or before this date are installed automatically. Date is formatted as `YYYY-MM-DD`. Conflicts with `approve_after_days`
    #[builder(into)]
    #[serde(rename = "approveUntilDate")]
    pub r#approve_until_date: Box<String>,
    /// Compliance level for patches approved by this rule.
    #[builder(into)]
    #[serde(rename = "complianceLevel")]
    pub r#compliance_level: Box<String>,
    /// Boolean enabling the application of non-security updates.
    #[builder(into)]
    #[serde(rename = "enableNonSecurity")]
    pub r#enable_non_security: Box<bool>,
    /// Patch filter group that defines the criteria for the rule.
    #[builder(into)]
    #[serde(rename = "patchFilters")]
    pub r#patch_filters: Box<Vec<super::super::types::ssm::GetPatchBaselineApprovalRulePatchFilter>>,
}
