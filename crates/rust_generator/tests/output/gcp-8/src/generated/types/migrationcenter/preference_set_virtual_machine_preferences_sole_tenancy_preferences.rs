#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreferenceSetVirtualMachinePreferencesSoleTenancyPreferences {
    /// Commitment plan to consider when calculating costs for virtual machine insights and recommendations. If you are unsure which value to set, a 3 year commitment plan is often a good value to start with. Possible values: `COMMITMENT_PLAN_UNSPECIFIED`, `ON_DEMAND`, `COMMITMENT_1_YEAR`, `COMMITMENT_3_YEAR`
    #[builder(into, default)]
    #[serde(rename = "commitmentPlan")]
    pub r#commitment_plan: Box<Option<String>>,
    /// CPU overcommit ratio. Acceptable values are between 1.0 and 2.0 inclusive.
    #[builder(into, default)]
    #[serde(rename = "cpuOvercommitRatio")]
    pub r#cpu_overcommit_ratio: Box<Option<f64>>,
    /// Sole Tenancy nodes maintenance policy. Possible values: `HOST_MAINTENANCE_POLICY_UNSPECIFIED`, `HOST_MAINTENANCE_POLICY_DEFAULT`, `HOST_MAINTENANCE_POLICY_RESTART_IN_PLACE`, `HOST_MAINTENANCE_POLICY_MIGRATE_WITHIN_NODE_GROUP`
    #[builder(into, default)]
    #[serde(rename = "hostMaintenancePolicy")]
    pub r#host_maintenance_policy: Box<Option<String>>,
    /// A list of sole tenant node types. An empty list means that all possible node types will be considered.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "nodeTypes")]
    pub r#node_types: Box<Option<Vec<super::super::types::migrationcenter::PreferenceSetVirtualMachinePreferencesSoleTenancyPreferencesNodeType>>>,
}
