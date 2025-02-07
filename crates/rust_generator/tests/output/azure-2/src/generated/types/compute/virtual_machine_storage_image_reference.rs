#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualMachineStorageImageReference {
    /// Specifies the ID of the Custom Image which the Virtual Machine should be created from. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Specifies the offer of the image used to create the virtual machine. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "offer")]
    pub r#offer: Box<Option<String>>,
    /// Specifies the publisher of the image used to create the virtual machine. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "publisher")]
    pub r#publisher: Box<Option<String>>,
    /// Specifies the SKU of the image used to create the virtual machine. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "sku")]
    pub r#sku: Box<Option<String>>,
    /// Specifies the version of the image used to create the virtual machine. Changing this forces a new resource to be created.
    /// 
    /// To provision a Custom Image, the following fields are applicable:
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
