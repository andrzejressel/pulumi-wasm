#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BackendCredentialsAuthorization {
    /// The authentication Parameter value.
    #[builder(into, default)]
    #[serde(rename = "parameter")]
    pub r#parameter: Box<Option<String>>,
    /// The authentication Scheme name.
    #[builder(into, default)]
    #[serde(rename = "scheme")]
    pub r#scheme: Box<Option<String>>,
}
