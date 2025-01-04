#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OrchestratedVirtualMachineScaleSetSkuProfile {
    /// Specifies the allocation strategy for the virtual machine scale set based on which the VMs will be allocated. Possible values are `LowestPrice` and `CapacityOptimized`.
    #[builder(into)]
    #[serde(rename = "allocationStrategy")]
    pub r#allocation_strategy: Box<String>,
    /// Specifies the VM sizes for the virtual machine scale set.
    #[builder(into)]
    #[serde(rename = "vmSizes")]
    pub r#vm_sizes: Box<Vec<String>>,
}
