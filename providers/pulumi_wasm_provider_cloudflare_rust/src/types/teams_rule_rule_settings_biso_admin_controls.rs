#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettingsBisoAdminControls {
    #[serde(rename = "disableCopyPaste")]
    pub r#disable_copy_paste: Box<Option<bool>>,
    #[serde(rename = "disableDownload")]
    pub r#disable_download: Box<Option<bool>>,
    #[serde(rename = "disableKeyboard")]
    pub r#disable_keyboard: Box<Option<bool>>,
    #[serde(rename = "disablePrinting")]
    pub r#disable_printing: Box<Option<bool>>,
    #[serde(rename = "disableUpload")]
    pub r#disable_upload: Box<Option<bool>>,
}
