#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CertificateCertificatePolicyLifetimeActionTrigger {
    /// The number of days before the Certificate expires that the action associated with this Trigger should run. Conflicts with `lifetime_percentage`.
    #[builder(into, default)]
    #[serde(rename = "daysBeforeExpiry")]
    pub r#days_before_expiry: Box<Option<i32>>,
    /// The percentage at which during the Certificates Lifetime the action associated with this Trigger should run. Conflicts with `days_before_expiry`.
    #[builder(into, default)]
    #[serde(rename = "lifetimePercentage")]
    pub r#lifetime_percentage: Box<Option<i32>>,
}