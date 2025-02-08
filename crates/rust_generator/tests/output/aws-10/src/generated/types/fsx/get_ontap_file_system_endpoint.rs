#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetOntapFileSystemEndpoint {
    /// A FileSystemEndpoint for managing your file system by setting up NetApp SnapMirror with other ONTAP systems. See FileSystemEndpoint below.
    #[builder(into)]
    #[serde(rename = "interclusters")]
    pub r#interclusters: Box<Vec<super::super::types::fsx::GetOntapFileSystemEndpointIntercluster>>,
    /// A FileSystemEndpoint for managing your file system using the NetApp ONTAP CLI and NetApp ONTAP API. See FileSystemEndpoint below.
    #[builder(into)]
    #[serde(rename = "managements")]
    pub r#managements: Box<Vec<super::super::types::fsx::GetOntapFileSystemEndpointManagement>>,
}
