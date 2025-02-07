#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceFromMachineImageConfidentialInstanceConfig {
    /// The confidential computing technology the instance uses.
    /// SEV is an AMD feature. TDX is an Intel feature. One of the following
    /// values is required: SEV, SEV_SNP, TDX. If SEV_SNP, min_cpu_platform =
    /// "AMD Milan" is currently required.
    #[builder(into, default)]
    #[serde(rename = "confidentialInstanceType")]
    pub r#confidential_instance_type: Box<Option<String>>,
    /// Defines whether the instance should have confidential compute enabled. Field will be deprecated in a future release
    #[builder(into, default)]
    #[serde(rename = "enableConfidentialCompute")]
    pub r#enable_confidential_compute: Box<Option<bool>>,
}
