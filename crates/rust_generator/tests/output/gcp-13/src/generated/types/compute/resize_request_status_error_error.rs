#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResizeRequestStatusErrorError {
    /// (Output)
    /// [Output Only] The error type identifier for this error.
    #[builder(into, default)]
    #[serde(rename = "code")]
    pub r#code: Box<Option<String>>,
    /// (Output)
    /// [Output Only] An optional list of messages that contain the error details. There is a set of defined message types to use for providing details.The syntax depends on the error code. For example, QuotaExceededInfo will have details when the error code is QUOTA_EXCEEDED.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "errorDetails")]
    pub r#error_details: Box<Option<Vec<super::super::types::compute::ResizeRequestStatusErrorErrorErrorDetail>>>,
    /// (Output)
    /// Output Only] Indicates the field in the request that caused the error. This property is optional.
    #[builder(into, default)]
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// (Output)
    /// The localized error message in the above locale.
    #[builder(into, default)]
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
}
