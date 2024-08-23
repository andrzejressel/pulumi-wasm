#[derive(serde::Serialize)]
pub struct AccessIdentityProviderScimConfig {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "groupMemberDeprovision")]
    pub r#group_member_deprovision: Box<Option<bool>>,
    #[serde(rename = "seatDeprovision")]
    pub r#seat_deprovision: Box<Option<bool>>,
    #[serde(rename = "secret")]
    pub r#secret: Box<Option<String>>,
    #[serde(rename = "userDeprovision")]
    pub r#user_deprovision: Box<Option<bool>>,
}
