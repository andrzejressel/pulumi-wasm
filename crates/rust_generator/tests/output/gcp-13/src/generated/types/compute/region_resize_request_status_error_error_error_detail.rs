#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RegionResizeRequestStatusErrorErrorErrorDetail {
    /// (Output)
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "errorInfos")]
    pub r#error_infos: Box<Option<Vec<super::super::types::compute::RegionResizeRequestStatusErrorErrorErrorDetailErrorInfo>>>,
    /// (Output)
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "helps")]
    pub r#helps: Box<Option<Vec<super::super::types::compute::RegionResizeRequestStatusErrorErrorErrorDetailHelp>>>,
    /// (Output)
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "localizedMessages")]
    pub r#localized_messages: Box<Option<Vec<super::super::types::compute::RegionResizeRequestStatusErrorErrorErrorDetailLocalizedMessage>>>,
    /// (Output)
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "quotaInfos")]
    pub r#quota_infos: Box<Option<Vec<super::super::types::compute::RegionResizeRequestStatusErrorErrorErrorDetailQuotaInfo>>>,
}
