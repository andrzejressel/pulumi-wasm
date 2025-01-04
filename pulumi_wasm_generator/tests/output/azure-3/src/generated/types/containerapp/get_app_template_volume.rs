#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAppTemplateVolume {
    /// The name of the Container App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The name of the `AzureFile` storage.
    #[builder(into)]
    #[serde(rename = "storageName")]
    pub r#storage_name: Box<String>,
    /// The type of storage volume. Possible values include `AzureFile` and `EmptyDir`. Defaults to `EmptyDir`.
    #[builder(into)]
    #[serde(rename = "storageType")]
    pub r#storage_type: Box<String>,
}
