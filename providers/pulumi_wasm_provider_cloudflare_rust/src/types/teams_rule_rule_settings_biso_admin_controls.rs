#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct TeamsRuleRuleSettingsBisoAdminControls {
    /// Disable clipboard redirection.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "disableClipboardRedirection")]
    pub r#disable_clipboard_redirection: Box<Option<bool>>,
    /// Disable copy-paste.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "disableCopyPaste")]
    pub r#disable_copy_paste: Box<Option<bool>>,
    /// Disable download.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "disableDownload")]
    pub r#disable_download: Box<Option<bool>>,
    /// Disable keyboard usage.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "disableKeyboard")]
    pub r#disable_keyboard: Box<Option<bool>>,
    /// Disable printing.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "disablePrinting")]
    pub r#disable_printing: Box<Option<bool>>,
    /// Disable upload.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "disableUpload")]
    pub r#disable_upload: Box<Option<bool>>,
}
