#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkflowTemplatePlacementManagedClusterConfigWorkerConfigAccelerator {
    /// The number of the accelerator cards of this type exposed to this instance.
    #[builder(into, default)]
    #[serde(rename = "acceleratorCount")]
    pub r#accelerator_count: Box<Option<i32>>,
    /// Full URL, partial URI, or short name of the accelerator type resource to expose to this instance. See (https://cloud.google.com/dataproc/docs/concepts/configuring-clusters/auto-zone#using_auto_zone_placement) feature, you must use the short name of the accelerator type resource, for example, `nvidia-tesla-k80`.
    #[builder(into, default)]
    #[serde(rename = "acceleratorType")]
    pub r#accelerator_type: Box<Option<String>>,
}
