#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SecurityScanConfigAuthenticationCustomAccount {
    /// The login form URL of the website.
    #[builder(into)]
    #[serde(rename = "loginUrl")]
    pub r#login_url: Box<String>,
    /// The password of the custom account. The credential is stored encrypted
    /// in GCP.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// The user name of the custom account.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
