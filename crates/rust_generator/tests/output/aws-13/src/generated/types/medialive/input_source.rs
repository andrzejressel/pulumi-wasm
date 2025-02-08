#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InputSource {
    /// The key used to extract the password from EC2 Parameter store.
    #[builder(into)]
    #[serde(rename = "passwordParam")]
    pub r#password_param: Box<String>,
    /// The URL where the stream is pulled from.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<String>,
    /// The username for the input source.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
