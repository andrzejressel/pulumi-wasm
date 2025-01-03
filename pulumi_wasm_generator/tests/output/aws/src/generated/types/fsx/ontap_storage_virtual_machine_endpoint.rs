#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OntapStorageVirtualMachineEndpoint {
    /// An endpoint for accessing data on your storage virtual machine via iSCSI protocol. See Endpoint.
    #[builder(into, default)]
    #[serde(rename = "iscsis")]
    pub r#iscsis: Box<Option<Vec<super::super::types::fsx::OntapStorageVirtualMachineEndpointIscsi>>>,
    /// An endpoint for managing your file system using the NetApp ONTAP CLI and NetApp ONTAP API. See Endpoint.
    #[builder(into, default)]
    #[serde(rename = "managements")]
    pub r#managements: Box<Option<Vec<super::super::types::fsx::OntapStorageVirtualMachineEndpointManagement>>>,
    /// An endpoint for accessing data on your storage virtual machine via NFS protocol. See Endpoint.
    #[builder(into, default)]
    #[serde(rename = "nfs")]
    pub r#nfs: Box<Option<Vec<super::super::types::fsx::OntapStorageVirtualMachineEndpointNf>>>,
    /// An endpoint for accessing data on your storage virtual machine via SMB protocol. This is only set if an active_directory_configuration has been set. See Endpoint.
    #[builder(into, default)]
    #[serde(rename = "smbs")]
    pub r#smbs: Box<Option<Vec<super::super::types::fsx::OntapStorageVirtualMachineEndpointSmb>>>,
}
