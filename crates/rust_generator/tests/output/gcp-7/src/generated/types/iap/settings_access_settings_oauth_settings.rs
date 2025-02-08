#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SettingsAccessSettingsOauthSettings {
    /// Domain hint to send as hd=? parameter in OAuth request flow.
    /// Enables redirect to primary IDP by skipping Google's login screen.
    /// (https://developers.google.com/identity/protocols/OpenIDConnect#hd-param)
    /// Note: IAP does not verify that the id token's hd claim matches this value
    /// since access behavior is managed by IAM policies.
    #[builder(into, default)]
    #[serde(rename = "loginHint")]
    pub r#login_hint: Box<Option<String>>,
    /// List of client ids allowed to use IAP programmatically.
    #[builder(into, default)]
    #[serde(rename = "programmaticClients")]
    pub r#programmatic_clients: Box<Option<Vec<String>>>,
}
