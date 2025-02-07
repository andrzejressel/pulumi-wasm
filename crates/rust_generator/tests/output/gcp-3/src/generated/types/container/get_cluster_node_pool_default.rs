#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterNodePoolDefault {
    /// Subset of NodeConfig message that has defaults.
    #[builder(into)]
    #[serde(rename = "nodeConfigDefaults")]
    pub r#node_config_defaults: Box<Vec<super::super::types::container::GetClusterNodePoolDefaultNodeConfigDefault>>,
}
