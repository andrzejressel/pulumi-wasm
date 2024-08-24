#[derive(serde::Serialize)]
pub struct GetDevicesDevice {
    /// When the device was created.
    #[serde(rename = "created")]
    pub r#created: Box<Option<String>>,
    /// Whether the device has been deleted.
    #[serde(rename = "deleted")]
    pub r#deleted: Box<Option<bool>>,
    /// The type of the device.
    #[serde(rename = "deviceType")]
    pub r#device_type: Box<Option<String>>,
    /// Device ID.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// IPv4 or IPv6 address.
    #[serde(rename = "ip")]
    pub r#ip: Box<Option<String>>,
    /// The device's public key.
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    /// When the device was last seen.
    #[serde(rename = "lastSeen")]
    pub r#last_seen: Box<Option<String>>,
    /// The device's MAC address.
    #[serde(rename = "macAddress")]
    pub r#mac_address: Box<Option<String>>,
    /// The device manufacturer's name.
    #[serde(rename = "manufacturer")]
    pub r#manufacturer: Box<Option<String>>,
    /// The device model name.
    #[serde(rename = "model")]
    pub r#model: Box<Option<String>>,
    /// The device name.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The Linux distribution name.
    #[serde(rename = "osDistroName")]
    pub r#os_distro_name: Box<Option<String>>,
    /// The Linux distribution revision.
    #[serde(rename = "osDistroRevision")]
    pub r#os_distro_revision: Box<Option<String>>,
    /// The operating system version.
    #[serde(rename = "osVersion")]
    pub r#os_version: Box<Option<String>>,
    /// When the device was revoked.
    #[serde(rename = "revokedAt")]
    pub r#revoked_at: Box<Option<String>>,
    /// The device's serial number.
    #[serde(rename = "serialNumber")]
    pub r#serial_number: Box<Option<String>>,
    /// When the device was updated.
    #[serde(rename = "updated")]
    pub r#updated: Box<Option<String>>,
    /// User's email.
    #[serde(rename = "userEmail")]
    pub r#user_email: Box<Option<String>>,
    /// User's ID.
    #[serde(rename = "userId")]
    pub r#user_id: Box<Option<String>>,
    /// User's Name.
    #[serde(rename = "userName")]
    pub r#user_name: Box<Option<String>>,
    /// The WARP client version.
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
