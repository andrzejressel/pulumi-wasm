#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LocationFsxOntapFileSystemProtocol {
    /// Network File System (NFS) protocol that DataSync uses to access your FSx ONTAP file system. See NFS below.
    #[builder(into, default)]
    #[serde(rename = "nfs")]
    pub r#nfs: Box<Option<super::super::types::datasync::LocationFsxOntapFileSystemProtocolNfs>>,
    /// Server Message Block (SMB) protocol that DataSync uses to access your FSx ONTAP file system. See [SMB] (#smb) below.
    #[builder(into, default)]
    #[serde(rename = "smb")]
    pub r#smb: Box<Option<super::super::types::datasync::LocationFsxOntapFileSystemProtocolSmb>>,
}
