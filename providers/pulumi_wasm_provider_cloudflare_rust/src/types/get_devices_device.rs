#[derive(serde::Serialize)]
pub struct GetDevicesDevice {
    #[serde(rename = "created")]
    pub r#created: Box<Option<String>>,
    #[serde(rename = "deleted")]
    pub r#deleted: Box<Option<bool>>,
    #[serde(rename = "deviceType")]
    pub r#device_type: Box<Option<String>>,
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "ip")]
    pub r#ip: Box<Option<String>>,
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    #[serde(rename = "lastSeen")]
    pub r#last_seen: Box<Option<String>>,
    #[serde(rename = "macAddress")]
    pub r#mac_address: Box<Option<String>>,
    #[serde(rename = "manufacturer")]
    pub r#manufacturer: Box<Option<String>>,
    #[serde(rename = "model")]
    pub r#model: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    #[serde(rename = "osDistroName")]
    pub r#os_distro_name: Box<Option<String>>,
    #[serde(rename = "osDistroRevision")]
    pub r#os_distro_revision: Box<Option<String>>,
    #[serde(rename = "osVersion")]
    pub r#os_version: Box<Option<String>>,
    #[serde(rename = "revokedAt")]
    pub r#revoked_at: Box<Option<String>>,
    #[serde(rename = "serialNumber")]
    pub r#serial_number: Box<Option<String>>,
    #[serde(rename = "updated")]
    pub r#updated: Box<Option<String>>,
    #[serde(rename = "userEmail")]
    pub r#user_email: Box<Option<String>>,
    #[serde(rename = "userId")]
    pub r#user_id: Box<Option<String>>,
    #[serde(rename = "userName")]
    pub r#user_name: Box<Option<String>>,
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}