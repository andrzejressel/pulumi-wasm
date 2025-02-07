#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LaunchTemplateCpuOptions {
    /// Indicates whether to enable the instance for AMD SEV-SNP. AMD SEV-SNP is supported with M6a, R6a, and C6a instance types only. Valid values are `enabled` and `disabled`.
    #[builder(into, default)]
    #[serde(rename = "amdSevSnp")]
    pub r#amd_sev_snp: Box<Option<String>>,
    /// The number of CPU cores for the instance.
    #[builder(into, default)]
    #[serde(rename = "coreCount")]
    pub r#core_count: Box<Option<i32>>,
    /// The number of threads per CPU core.
    /// To disable Intel Hyper-Threading Technology for the instance, specify a value of 1.
    /// Otherwise, specify the default value of 2.
    /// 
    /// Both number of CPU cores and threads per core must be specified. Valid number of CPU cores and threads per core for the instance type can be found in the [CPU Options Documentation](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-optimize-cpu.html?shortFooter=true#cpu-options-supported-instances-values)
    #[builder(into, default)]
    #[serde(rename = "threadsPerCore")]
    pub r#threads_per_core: Box<Option<i32>>,
}
