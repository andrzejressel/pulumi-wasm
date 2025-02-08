#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSubscriptionPushConfigOidcToken {
    /// Audience to be used when generating OIDC token. The audience claim
    /// identifies the recipients that the JWT is intended for. The audience
    /// value is a single case-sensitive string. Having multiple values (array)
    /// for the audience field is not supported. More info about the OIDC JWT
    /// token audience here: https://tools.ietf.org/html/rfc7519#section-4.1.3
    /// Note: if not specified, the Push endpoint URL will be used.
    #[builder(into)]
    #[serde(rename = "audience")]
    pub r#audience: Box<String>,
    /// Service account email to be used for generating the OIDC token.
    /// The caller (for subscriptions.create, subscriptions.patch, and
    /// subscriptions.modifyPushConfig RPCs) must have the
    /// iam.serviceAccounts.actAs permission for the service account.
    #[builder(into)]
    #[serde(rename = "serviceAccountEmail")]
    pub r#service_account_email: Box<String>,
}
