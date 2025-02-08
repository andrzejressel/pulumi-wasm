#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceAuth {
    /// The password
    #[builder(into, default)]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// The address of the server for the authentication
    #[builder(into)]
    #[serde(rename = "serverAddress")]
    pub r#server_address: Box<String>,
    /// The username
    #[builder(into, default)]
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
