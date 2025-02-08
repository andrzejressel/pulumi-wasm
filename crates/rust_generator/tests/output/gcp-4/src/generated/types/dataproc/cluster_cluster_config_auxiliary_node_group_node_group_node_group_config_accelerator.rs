#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterClusterConfigAuxiliaryNodeGroupNodeGroupNodeGroupConfigAccelerator {
    /// The number of the accelerator cards of this type exposed to this instance. Often restricted to one of `1`, `2`, `4`, or `8`.
    /// 
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "acceleratorCount")]
    pub r#accelerator_count: Box<i32>,
    /// The short name of the accelerator type to expose to this instance. For example, `nvidia-tesla-k80`.
    #[builder(into)]
    #[serde(rename = "acceleratorType")]
    pub r#accelerator_type: Box<String>,
}
