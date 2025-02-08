#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NodePoolNodeConfigAdvancedMachineFeatures {
    /// Whether the node should have nested virtualization enabled.
    #[builder(into, default)]
    #[serde(rename = "enableNestedVirtualization")]
    pub r#enable_nested_virtualization: Box<Option<bool>>,
    /// The number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed.
    #[builder(into)]
    #[serde(rename = "threadsPerCore")]
    pub r#threads_per_core: Box<i32>,
}
