#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ResizeRequestStatusErrorErrorErrorDetail {
    /// (Output)
    /// [Output Only]
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "errorInfos")]
    pub r#error_infos: Box<Option<Vec<super::super::types::compute::ResizeRequestStatusErrorErrorErrorDetailErrorInfo>>>,
    /// (Output)
    /// [Output Only]
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "helps")]
    pub r#helps: Box<Option<Vec<super::super::types::compute::ResizeRequestStatusErrorErrorErrorDetailHelp>>>,
    /// (Output)
    /// [Output Only]
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "localizedMessages")]
    pub r#localized_messages: Box<Option<Vec<super::super::types::compute::ResizeRequestStatusErrorErrorErrorDetailLocalizedMessage>>>,
    /// (Output)
    /// [Output Only]
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "quotaInfos")]
    pub r#quota_infos: Box<Option<Vec<super::super::types::compute::ResizeRequestStatusErrorErrorErrorDetailQuotaInfo>>>,
}
