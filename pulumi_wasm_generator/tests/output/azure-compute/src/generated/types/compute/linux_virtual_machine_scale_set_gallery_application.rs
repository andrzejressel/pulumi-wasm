#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinuxVirtualMachineScaleSetGalleryApplication {
    /// Specifies the URI to an Azure Blob that will replace the default configuration for the package if provided. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "configurationBlobUri")]
    pub r#configuration_blob_uri: Box<Option<String>>,
    /// Specifies the order in which the packages have to be installed. Possible values are between `0` and `2147483647`. Defaults to `0`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "order")]
    pub r#order: Box<Option<i32>>,
    /// Specifies a passthrough value for more generic context. This field can be any valid `string` value. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "tag")]
    pub r#tag: Box<Option<String>>,
    /// Specifies the Gallery Application Version resource ID. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "versionId")]
    pub r#version_id: Box<String>,
}
