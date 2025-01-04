#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterClusterConfigMasterConfigAccelerator {
    /// The number of the accelerator cards of this type exposed to this instance. Often restricted to one of `1`, `2`, `4`, or `8`.
    /// 
    /// > The Cloud Dataproc API can return unintuitive error messages when using accelerators; even when you have defined an accelerator, Auto Zone Placement does not exclusively select
    /// zones that have that accelerator available. If you get a 400 error that the accelerator can't be found, this is a likely cause. Make sure you check [accelerator availability by zone](https://cloud.google.com/compute/docs/reference/rest/v1/acceleratorTypes/list)
    /// if you are trying to use accelerators in a given zone.
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
