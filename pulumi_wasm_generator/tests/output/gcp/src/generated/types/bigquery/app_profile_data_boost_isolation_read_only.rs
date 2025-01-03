#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AppProfileDataBoostIsolationReadOnly {
    /// The Compute Billing Owner for this Data Boost App Profile.
    /// Possible values are: `HOST_PAYS`.
    #[builder(into)]
    #[serde(rename = "computeBillingOwner")]
    pub r#compute_billing_owner: Box<String>,
}
