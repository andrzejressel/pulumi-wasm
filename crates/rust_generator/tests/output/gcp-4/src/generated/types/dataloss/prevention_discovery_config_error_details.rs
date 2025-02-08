#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PreventionDiscoveryConfigErrorDetails {
    /// The status code, which should be an enum value of google.rpc.Code.
    #[builder(into, default)]
    #[serde(rename = "code")]
    pub r#code: Box<Option<i32>>,
    /// A list of messages that carry the error details.
    #[builder(into, default)]
    #[serde(rename = "details")]
    pub r#details: Box<Option<Vec<std::collections::HashMap<String, String>>>>,
    /// A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client.
    #[builder(into, default)]
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
}
