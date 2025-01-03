#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FeatureSpecClusterupgradeGkeUpgradeOverridePostConditions {
    /// Amount of time to "soak" after a rollout has been finished before marking it COMPLETE. Cannot exceed 30 days.
    #[builder(into)]
    #[serde(rename = "soaking")]
    pub r#soaking: Box<String>,
}
