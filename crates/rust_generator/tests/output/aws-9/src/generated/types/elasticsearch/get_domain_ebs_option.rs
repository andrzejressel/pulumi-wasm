#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetDomainEbsOption {
    /// Whether EBS volumes are attached to data nodes in the domain.
    #[builder(into)]
    #[serde(rename = "ebsEnabled")]
    pub r#ebs_enabled: Box<bool>,
    /// The baseline input/output (I/O) performance of EBS volumes attached to data nodes.
    #[builder(into)]
    #[serde(rename = "iops")]
    pub r#iops: Box<i32>,
    /// The throughput (in MiB/s) of the EBS volumes attached to data nodes.
    #[builder(into)]
    #[serde(rename = "throughput")]
    pub r#throughput: Box<i32>,
    /// The size of EBS volumes attached to data nodes (in GB).
    #[builder(into)]
    #[serde(rename = "volumeSize")]
    pub r#volume_size: Box<i32>,
    /// The type of EBS volumes attached to data nodes.
    #[builder(into)]
    #[serde(rename = "volumeType")]
    pub r#volume_type: Box<String>,
}
