#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualMachineBootDiagnostics {
    /// Should Boot Diagnostics be enabled for this Virtual Machine?
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// The Storage Account's Blob Endpoint which should hold the virtual machine's diagnostic files.
    /// 
    /// > **NOTE:** This needs to be the root of a Storage Account and not a Storage Container.
    #[builder(into)]
    #[serde(rename = "storageUri")]
    pub r#storage_uri: Box<String>,
}
