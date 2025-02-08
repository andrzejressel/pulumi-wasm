#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetInstanceTemplateConfidentialInstanceConfig {
    /// The confidential computing technology the instance uses.
    /// SEV is an AMD feature. TDX is an Intel feature. One of the following
    /// values is required: SEV, SEV_SNP, TDX. If SEV_SNP, min_cpu_platform =
    /// "AMD Milan" is currently required.
    #[builder(into)]
    #[serde(rename = "confidentialInstanceType")]
    pub r#confidential_instance_type: Box<String>,
    /// Defines whether the instance should have confidential compute enabled. `on_host_maintenance` has to be set to TERMINATE or this will fail to create the VM.
    #[builder(into)]
    #[serde(rename = "enableConfidentialCompute")]
    pub r#enable_confidential_compute: Box<bool>,
}
