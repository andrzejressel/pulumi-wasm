#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ManagedUserPoolClientTokenValidityUnits {
    /// Time unit for the value in `access_token_validity` and defaults to `hours`.
    #[builder(into, default)]
    #[serde(rename = "accessToken")]
    pub r#access_token: Box<Option<String>>,
    /// Time unit for the value in `id_token_validity`, and it defaults to `hours`.
    #[builder(into, default)]
    #[serde(rename = "idToken")]
    pub r#id_token: Box<Option<String>>,
    /// Time unit for the value in `refresh_token_validity` and defaults to `days`.
    #[builder(into, default)]
    #[serde(rename = "refreshToken")]
    pub r#refresh_token: Box<Option<String>>,
}
