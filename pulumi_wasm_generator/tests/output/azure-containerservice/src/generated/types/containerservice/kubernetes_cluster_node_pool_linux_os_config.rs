#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KubernetesClusterNodePoolLinuxOsConfig {
    /// Specifies the size of swap file on each node in MB. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "swapFileSizeMb")]
    pub r#swap_file_size_mb: Box<Option<i32>>,
    /// A `sysctl_config` block as defined below. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "sysctlConfig")]
    pub r#sysctl_config: Box<Option<super::super::types::containerservice::KubernetesClusterNodePoolLinuxOsConfigSysctlConfig>>,
    /// specifies the defrag configuration for Transparent Huge Page. Possible values are `always`, `defer`, `defer+madvise`, `madvise` and `never`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "transparentHugePageDefrag")]
    pub r#transparent_huge_page_defrag: Box<Option<String>>,
    /// Specifies the Transparent Huge Page enabled configuration. Possible values are `always`, `madvise` and `never`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "transparentHugePageEnabled")]
    pub r#transparent_huge_page_enabled: Box<Option<String>>,
}