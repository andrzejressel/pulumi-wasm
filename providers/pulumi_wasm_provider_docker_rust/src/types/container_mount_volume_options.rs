#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ContainerMountVolumeOptions {
    /// Name of the driver to use to create the volume.
    #[serde(rename = "driverName")]
    pub r#driver_name: Box<Option<String>>,
    /// key/value map of driver specific options.
    #[serde(rename = "driverOptions")]
    pub r#driver_options: Box<Option<std::collections::HashMap<String, String>>>,
    /// User-defined key/value metadata.
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<Vec<crate::types::ContainerMountVolumeOptionsLabel>>>,
    /// Populate volume with data from the target.
    #[serde(rename = "noCopy")]
    pub r#no_copy: Box<Option<bool>>,
}
