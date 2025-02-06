#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSQuotaInfosQuotaInfoQuotaIncreaseEligibility {
    /// The enumeration of reasons when it is ineligible to request increase adjustment.
    #[builder(into)]
    #[serde(rename = "ineligibilityReason")]
    pub r#ineligibility_reason: Box<String>,
    /// Whether a higher quota value can be requested for the quota.
    #[builder(into)]
    #[serde(rename = "isEligible")]
    pub r#is_eligible: Box<bool>,
}
