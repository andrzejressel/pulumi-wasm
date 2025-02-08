#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct StackUserSetting {
    /// Action that is enabled or disabled.
    /// Valid values are `AUTO_TIME_ZONE_REDIRECTION`, `CLIPBOARD_COPY_FROM_LOCAL_DEVICE`, `CLIPBOARD_COPY_TO_LOCAL_DEVICE`, `DOMAIN_PASSWORD_SIGNIN`, `DOMAIN_SMART_CARD_SIGNIN`, `FILE_UPLOAD`, `FILE_DOWNLOAD`, or `PRINTING_TO_LOCAL_DEVICE`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// Whether the action is enabled or disabled.
    /// Valid values are `ENABLED` or `DISABLED`.
    #[builder(into)]
    #[serde(rename = "permission")]
    pub r#permission: Box<String>,
}
