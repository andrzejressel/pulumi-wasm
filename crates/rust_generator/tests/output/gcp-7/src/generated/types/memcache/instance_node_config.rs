#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceNodeConfig {
    /// Number of CPUs per node.
    #[builder(into)]
    #[serde(rename = "cpuCount")]
    pub r#cpu_count: Box<i32>,
    /// Memory size in Mebibytes for each memcache node.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "memorySizeMb")]
    pub r#memory_size_mb: Box<i32>,
}
