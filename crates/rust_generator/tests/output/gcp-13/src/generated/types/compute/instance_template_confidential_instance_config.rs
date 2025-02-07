#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceTemplateConfidentialInstanceConfig {
    /// Defines the confidential computing technology the instance uses. SEV is an AMD feature. TDX is an Intel feature. One of the following values is required: `SEV`, `SEV_SNP`, `TDX`. `on_host_maintenance` can be set to MIGRATE if `confidential_instance_type` is set to `SEV` and `min_cpu_platform` is set to `"AMD Milan"`. Otherwise, `on_host_maintenance` has to be set to TERMINATE or this will fail to create the VM. If `SEV_SNP`, currently `min_cpu_platform` has to be set to `"AMD Milan"` or this will fail to create the VM.
    #[builder(into, default)]
    #[serde(rename = "confidentialInstanceType")]
    pub r#confidential_instance_type: Box<Option<String>>,
    /// Defines whether the instance should have confidential compute enabled with AMD SEV. If enabled, `on_host_maintenance` can be set to MIGRATE if `min_cpu_platform` is set to `"AMD Milan"`. Otherwise, `on_host_maintenance` has to be set to TERMINATE or this will fail to create the VM.
    #[builder(into, default)]
    #[serde(rename = "enableConfidentialCompute")]
    pub r#enable_confidential_compute: Box<Option<bool>>,
}
