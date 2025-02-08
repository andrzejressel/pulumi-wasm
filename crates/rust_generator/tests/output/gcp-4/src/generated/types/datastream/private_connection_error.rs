#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PrivateConnectionError {
    /// A list of messages that carry the error details.
    #[builder(into, default)]
    #[serde(rename = "details")]
    pub r#details: Box<Option<std::collections::HashMap<String, String>>>,
    /// A message containing more information about the error that occurred.
    #[builder(into, default)]
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
}
