#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterCoreInstanceFleetLaunchSpecificationsOnDemandSpecification {
    /// Specifies the strategy to use in launching On-Demand instance fleets. Currently, the only option is `lowest-price` (the default), which launches the lowest price first.
    #[builder(into)]
    #[serde(rename = "allocationStrategy")]
    pub r#allocation_strategy: Box<String>,
}