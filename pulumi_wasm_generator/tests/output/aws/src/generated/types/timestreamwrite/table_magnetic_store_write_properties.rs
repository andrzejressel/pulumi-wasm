#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TableMagneticStoreWriteProperties {
    /// A flag to enable magnetic store writes.
    #[builder(into, default)]
    #[serde(rename = "enableMagneticStoreWrites")]
    pub r#enable_magnetic_store_writes: Box<Option<bool>>,
    /// The location to write error reports for records rejected asynchronously during magnetic store writes. See Magnetic Store Rejected Data Location below for more details.
    #[builder(into, default)]
    #[serde(rename = "magneticStoreRejectedDataLocation")]
    pub r#magnetic_store_rejected_data_location: Box<Option<super::super::types::timestreamwrite::TableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocation>>,
}
