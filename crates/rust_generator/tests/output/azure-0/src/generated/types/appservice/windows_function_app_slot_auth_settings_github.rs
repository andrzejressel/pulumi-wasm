#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsFunctionAppSlotAuthSettingsGithub {
    /// The ID of the GitHub app used for login.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<String>,
    /// The Client Secret of the GitHub app used for GitHub login. Cannot be specified with `client_secret_setting_name`.
    #[builder(into, default)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<Option<String>>,
    /// The app setting name that contains the `client_secret` value used for GitHub login. Cannot be specified with `client_secret`.
    #[builder(into, default)]
    #[serde(rename = "clientSecretSettingName")]
    pub r#client_secret_setting_name: Box<Option<String>>,
    /// an `oauth_scopes`.
    #[builder(into, default)]
    #[serde(rename = "oauthScopes")]
    pub r#oauth_scopes: Box<Option<Vec<String>>>,
}
