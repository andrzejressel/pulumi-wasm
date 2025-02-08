#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AiFeatureStoreOnlineServingConfig {
    /// The number of nodes for each cluster. The number of nodes will not scale automatically but can be scaled manually by providing different values when updating.
    #[builder(into, default)]
    #[serde(rename = "fixedNodeCount")]
    pub r#fixed_node_count: Box<Option<i32>>,
    /// Online serving scaling configuration. Only one of fixedNodeCount and scaling can be set. Setting one will reset the other.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "scaling")]
    pub r#scaling: Box<Option<super::super::types::vertex::AiFeatureStoreOnlineServingConfigScaling>>,
}
