#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainerRuntimePlatform {
    /// The vCPU architecture. The default value is X86_64. Valid values are X86_64 and ARM64.
    #[builder(into)]
    #[serde(rename = "cpuArchitecture")]
    pub r#cpu_architecture: Box<String>,
    /// The operating system for the compute environment. V
    #[builder(into)]
    #[serde(rename = "operatingSystemFamily")]
    pub r#operating_system_family: Box<String>,
}
