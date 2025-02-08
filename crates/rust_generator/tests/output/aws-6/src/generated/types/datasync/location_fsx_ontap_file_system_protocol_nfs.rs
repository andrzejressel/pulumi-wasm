#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LocationFsxOntapFileSystemProtocolNfs {
    /// Mount options that are available for DataSync to access an NFS location. See NFS Mount Options below.
    #[builder(into)]
    #[serde(rename = "mountOptions")]
    pub r#mount_options: Box<super::super::types::datasync::LocationFsxOntapFileSystemProtocolNfsMountOptions>,
}
