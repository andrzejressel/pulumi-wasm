#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PatchBaselineApprovalRule {
    /// Number of days after the release date of each patch matched by the rule the patch is marked as approved in the patch baseline. Valid Range: 0 to 360. Conflicts with `approve_until_date`.
    #[builder(into, default)]
    #[serde(rename = "approveAfterDays")]
    pub r#approve_after_days: Box<Option<i32>>,
    /// Cutoff date for auto approval of released patches. Any patches released on or before this date are installed automatically. Date is formatted as `YYYY-MM-DD`. Conflicts with `approve_after_days`
    #[builder(into, default)]
    #[serde(rename = "approveUntilDate")]
    pub r#approve_until_date: Box<Option<String>>,
    /// Compliance level for patches approved by this rule. Valid values are `CRITICAL`, `HIGH`, `MEDIUM`, `LOW`, `INFORMATIONAL`, and `UNSPECIFIED`. The default value is `UNSPECIFIED`.
    #[builder(into, default)]
    #[serde(rename = "complianceLevel")]
    pub r#compliance_level: Box<Option<String>>,
    /// Boolean enabling the application of non-security updates. The default value is `false`. Valid for Linux instances only.
    #[builder(into, default)]
    #[serde(rename = "enableNonSecurity")]
    pub r#enable_non_security: Box<Option<bool>>,
    /// Patch filter group that defines the criteria for the rule. Up to 5 patch filters can be specified per approval rule using Key/Value pairs. Valid combinations of these Keys and the `operating_system` value can be found in the [SSM DescribePatchProperties API Reference](https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_DescribePatchProperties.html). Valid Values are exact values for the patch property given as the key, or a wildcard `*`, which matches all values. `PATCH_SET` defaults to `OS` if unspecified
    #[builder(into)]
    #[serde(rename = "patchFilters")]
    pub r#patch_filters: Box<Vec<super::super::types::ssm::PatchBaselineApprovalRulePatchFilter>>,
}
