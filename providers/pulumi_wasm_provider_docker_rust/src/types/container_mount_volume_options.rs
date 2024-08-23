#[derive(serde::Serialize)]
pub struct ContainerMountVolumeOptions {
    #[serde(rename = "driverName")]
    pub r#driver_name: Box<Option<String>>,
    #[serde(rename = "driverOptions")]
    pub r#driver_options: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<Vec<crate::types::ContainerMountVolumeOptionsLabel>>>,
    #[serde(rename = "noCopy")]
    pub r#no_copy: Box<Option<bool>>,
}
