#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetDevicesDevice {
    /// When the device was created.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "created")]
    pub r#created: Box<Option<String>>,
    /// Whether the device has been deleted.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "deleted")]
    pub r#deleted: Box<Option<bool>>,
    /// The type of the device.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "deviceType")]
    pub r#device_type: Box<Option<String>>,
    /// Device ID.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// IPv4 or IPv6 address.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "ip")]
    pub r#ip: Box<Option<String>>,
    /// The device's public key.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    /// When the device was last seen.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "lastSeen")]
    pub r#last_seen: Box<Option<String>>,
    /// The device's MAC address.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "macAddress")]
    pub r#mac_address: Box<Option<String>>,
    /// The device manufacturer's name.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "manufacturer")]
    pub r#manufacturer: Box<Option<String>>,
    /// The device model name.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "model")]
    pub r#model: Box<Option<String>>,
    /// The device name.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The Linux distribution name.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "osDistroName")]
    pub r#os_distro_name: Box<Option<String>>,
    /// The Linux distribution revision.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "osDistroRevision")]
    pub r#os_distro_revision: Box<Option<String>>,
    /// The operating system version.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "osVersion")]
    pub r#os_version: Box<Option<String>>,
    /// Extra version value following the operating system version.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "osVersionExtra")]
    pub r#os_version_extra: Box<Option<String>>,
    /// When the device was revoked.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "revokedAt")]
    pub r#revoked_at: Box<Option<String>>,
    /// The device's serial number.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "serialNumber")]
    pub r#serial_number: Box<Option<String>>,
    /// When the device was updated.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "updated")]
    pub r#updated: Box<Option<String>>,
    /// User's email.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "userEmail")]
    pub r#user_email: Box<Option<String>>,
    /// User's ID.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "userId")]
    pub r#user_id: Box<Option<String>>,
    /// User's Name.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "userName")]
    pub r#user_name: Box<Option<String>>,
    /// The WARP client version.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
