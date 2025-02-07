#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterNodePoolNodeConfigSoleTenantConfigNodeAffinity {
    /// The default or custom node affinity label key name.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Specifies affinity or anti-affinity. Accepted values are `"IN"` or `"NOT_IN"`
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// List of node affinity label values as strings.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
