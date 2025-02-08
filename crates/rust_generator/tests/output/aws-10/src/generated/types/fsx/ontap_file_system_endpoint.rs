#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OntapFileSystemEndpoint {
    /// An endpoint for managing your file system by setting up NetApp SnapMirror with other ONTAP systems. See Endpoint.
    #[builder(into, default)]
    #[serde(rename = "interclusters")]
    pub r#interclusters: Box<Option<Vec<super::super::types::fsx::OntapFileSystemEndpointIntercluster>>>,
    /// An endpoint for managing your file system using the NetApp ONTAP CLI and NetApp ONTAP API. See Endpoint.
    #[builder(into, default)]
    #[serde(rename = "managements")]
    pub r#managements: Box<Option<Vec<super::super::types::fsx::OntapFileSystemEndpointManagement>>>,
}
