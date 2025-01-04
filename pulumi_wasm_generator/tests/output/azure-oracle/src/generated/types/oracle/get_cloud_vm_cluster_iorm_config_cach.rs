#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCloudVmClusterIormConfigCach {
    /// A `db_plans` block as defined above.
    #[builder(into)]
    #[serde(rename = "dbPlans")]
    pub r#db_plans: Box<Vec<super::super::types::oracle::GetCloudVmClusterIormConfigCachDbPlan>>,
    /// Additional information about the current `lifecycleState`.
    #[builder(into)]
    #[serde(rename = "lifecycleDetails")]
    pub r#lifecycle_details: Box<String>,
    /// The current state of IORM configuration for the Exadata DB system.
    #[builder(into)]
    #[serde(rename = "lifecycleState")]
    pub r#lifecycle_state: Box<String>,
    /// The current value for the IORM objective. The default is `AUTO`.
    #[builder(into)]
    #[serde(rename = "objective")]
    pub r#objective: Box<String>,
}
