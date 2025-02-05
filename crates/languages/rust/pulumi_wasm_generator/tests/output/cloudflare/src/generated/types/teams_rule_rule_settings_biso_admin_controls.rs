#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TeamsRuleRuleSettingsBisoAdminControls {
    /// Disable clipboard redirection.
    #[builder(into, default)]
    #[serde(rename = "disableClipboardRedirection")]
    pub r#disable_clipboard_redirection: Box<Option<bool>>,
    /// Disable copy-paste.
    #[builder(into, default)]
    #[serde(rename = "disableCopyPaste")]
    pub r#disable_copy_paste: Box<Option<bool>>,
    /// Disable download.
    #[builder(into, default)]
    #[serde(rename = "disableDownload")]
    pub r#disable_download: Box<Option<bool>>,
    /// Disable keyboard usage.
    #[builder(into, default)]
    #[serde(rename = "disableKeyboard")]
    pub r#disable_keyboard: Box<Option<bool>>,
    /// Disable printing.
    #[builder(into, default)]
    #[serde(rename = "disablePrinting")]
    pub r#disable_printing: Box<Option<bool>>,
    /// Disable upload.
    #[builder(into, default)]
    #[serde(rename = "disableUpload")]
    pub r#disable_upload: Box<Option<bool>>,
}
