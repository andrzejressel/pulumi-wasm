#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterNodeConfigLocalNvmeSsdBlockConfig {
    /// Number of raw-block local NVMe SSD disks to be attached to the node. Each local SSD is 375 GB in size. If zero, it means no raw-block local NVMe SSD disks to be attached to the node.
    /// > Note: Local NVMe SSD storage available in GKE versions v1.25.3-gke.1800 and later.
    #[builder(into)]
    #[serde(rename = "localSsdCount")]
    pub r#local_ssd_count: Box<i32>,
}
