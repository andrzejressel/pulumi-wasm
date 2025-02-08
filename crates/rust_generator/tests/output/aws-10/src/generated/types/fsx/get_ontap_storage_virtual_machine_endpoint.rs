#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetOntapStorageVirtualMachineEndpoint {
    #[builder(into)]
    #[serde(rename = "iscsis")]
    pub r#iscsis: Box<Vec<super::super::types::fsx::GetOntapStorageVirtualMachineEndpointIscsi>>,
    /// An endpoint for managing SVMs using the NetApp ONTAP CLI, NetApp ONTAP API, or NetApp CloudManager. See SVM Endpoint below.
    #[builder(into)]
    #[serde(rename = "managements")]
    pub r#managements: Box<Vec<super::super::types::fsx::GetOntapStorageVirtualMachineEndpointManagement>>,
    /// An endpoint for connecting using the Network File System (NFS) protocol. See SVM Endpoint below.
    #[builder(into)]
    #[serde(rename = "nfs")]
    pub r#nfs: Box<Vec<super::super::types::fsx::GetOntapStorageVirtualMachineEndpointNf>>,
    /// An endpoint for connecting using the Server Message Block (SMB) protocol. See SVM Endpoint below.
    #[builder(into)]
    #[serde(rename = "smbs")]
    pub r#smbs: Box<Vec<super::super::types::fsx::GetOntapStorageVirtualMachineEndpointSmb>>,
}
