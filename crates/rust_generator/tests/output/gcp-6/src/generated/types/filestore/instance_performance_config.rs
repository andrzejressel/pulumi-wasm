#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstancePerformanceConfig {
    /// The instance will have a fixed provisioned IOPS value,
    /// which will remain constant regardless of instance
    /// capacity.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "fixedIops")]
    pub r#fixed_iops: Box<Option<super::super::types::filestore::InstancePerformanceConfigFixedIops>>,
    /// The instance provisioned IOPS will change dynamically
    /// based on the capacity of the instance.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "iopsPerTb")]
    pub r#iops_per_tb: Box<Option<super::super::types::filestore::InstancePerformanceConfigIopsPerTb>>,
}
