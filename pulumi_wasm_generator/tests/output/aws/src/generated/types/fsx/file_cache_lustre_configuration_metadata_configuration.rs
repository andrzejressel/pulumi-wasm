#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FileCacheLustreConfigurationMetadataConfiguration {
    /// The storage capacity of the Lustre MDT (Metadata Target) storage volume in gibibytes (GiB). The only supported value is `2400` GiB.
    #[builder(into)]
    #[serde(rename = "storageCapacity")]
    pub r#storage_capacity: Box<i32>,
}
