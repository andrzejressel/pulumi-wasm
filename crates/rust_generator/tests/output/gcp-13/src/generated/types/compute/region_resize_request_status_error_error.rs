#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionResizeRequestStatusErrorError {
    /// (Output)
    /// The error type identifier for this error.
    #[builder(into, default)]
    #[serde(rename = "code")]
    pub r#code: Box<Option<String>>,
    /// (Output)
    /// An array of messages that contain the error details. There is a set of defined message types to use for providing details.The syntax depends on the error code. For example, QuotaExceededInfo will have details when the error code is QUOTA_EXCEEDED.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "errorDetails")]
    pub r#error_details: Box<Option<Vec<super::super::types::compute::RegionResizeRequestStatusErrorErrorErrorDetail>>>,
    /// (Output)
    /// Indicates the field in the request that caused the error. This property is optional.
    #[builder(into, default)]
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// (Output)
    /// The localized error message in the above locale.
    #[builder(into, default)]
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
}
