#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTableMagneticStoreWriteProperty {
    /// Flag that is set based on if magnetic store writes are enabled.
    #[builder(into)]
    #[serde(rename = "enableMagneticStoreWrites")]
    pub r#enable_magnetic_store_writes: Box<bool>,
    /// Object containing the following attributes to describe error reports for records rejected during magnetic store writes.
    #[builder(into)]
    #[serde(rename = "magneticStoreRejectedDataLocations")]
    pub r#magnetic_store_rejected_data_locations: Box<Vec<super::super::types::timestreamwrite::GetTableMagneticStoreWritePropertyMagneticStoreRejectedDataLocation>>,
}
