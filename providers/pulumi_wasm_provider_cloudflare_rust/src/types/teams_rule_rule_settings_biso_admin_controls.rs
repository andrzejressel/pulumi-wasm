#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TeamsRuleRuleSettingsBisoAdminControls {
    /// Disable copy-paste.
    #[serde(rename = "disableCopyPaste")]
    pub r#disable_copy_paste: Box<Option<bool>>,
    /// Disable download.
    #[serde(rename = "disableDownload")]
    pub r#disable_download: Box<Option<bool>>,
    /// Disable keyboard usage.
    #[serde(rename = "disableKeyboard")]
    pub r#disable_keyboard: Box<Option<bool>>,
    /// Disable printing.
    #[serde(rename = "disablePrinting")]
    pub r#disable_printing: Box<Option<bool>>,
    /// Disable upload.
    #[serde(rename = "disableUpload")]
    pub r#disable_upload: Box<Option<bool>>,
}
