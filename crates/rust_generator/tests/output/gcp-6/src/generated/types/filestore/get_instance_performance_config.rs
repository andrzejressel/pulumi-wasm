#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInstancePerformanceConfig {
    /// The instance will have a fixed provisioned IOPS value,
    /// which will remain constant regardless of instance
    /// capacity.
    #[builder(into)]
    #[serde(rename = "fixedIops")]
    pub r#fixed_iops: Box<Vec<super::super::types::filestore::GetInstancePerformanceConfigFixedIop>>,
    /// The instance provisioned IOPS will change dynamically
    /// based on the capacity of the instance.
    #[builder(into)]
    #[serde(rename = "iopsPerTbs")]
    pub r#iops_per_tbs: Box<Vec<super::super::types::filestore::GetInstancePerformanceConfigIopsPerTb>>,
}
