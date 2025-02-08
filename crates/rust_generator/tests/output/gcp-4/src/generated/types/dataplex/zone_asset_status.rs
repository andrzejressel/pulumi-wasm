#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ZoneAssetStatus {
    /// Number of active assets.
    #[builder(into, default)]
    #[serde(rename = "activeAssets")]
    pub r#active_assets: Box<Option<i32>>,
    /// Number of assets that are in process of updating the security policy on attached resources.
    #[builder(into, default)]
    #[serde(rename = "securityPolicyApplyingAssets")]
    pub r#security_policy_applying_assets: Box<Option<i32>>,
    /// Output only. The time when the zone was last updated.
    #[builder(into, default)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Box<Option<String>>,
}
