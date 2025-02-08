#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CertificateManagedAuthorizationAttemptInfo {
    /// Human readable explanation for reaching the state. Provided to help
    /// address the configuration issues.
    /// Not guaranteed to be stable. For programmatic access use 'failure_reason' field.
    #[builder(into, default)]
    #[serde(rename = "details")]
    pub r#details: Box<Option<String>>,
    /// Domain name of the authorization attempt.
    #[builder(into, default)]
    #[serde(rename = "domain")]
    pub r#domain: Box<Option<String>>,
    /// Reason for failure of the authorization attempt for the domain.
    #[builder(into, default)]
    #[serde(rename = "failureReason")]
    pub r#failure_reason: Box<Option<String>>,
    /// State of the domain for managed certificate issuance.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
}
