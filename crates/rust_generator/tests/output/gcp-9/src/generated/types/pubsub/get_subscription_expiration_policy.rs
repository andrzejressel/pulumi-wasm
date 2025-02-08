#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSubscriptionExpirationPolicy {
    /// Specifies the "time-to-live" duration for an associated resource. The
    /// resource expires if it is not active for a period of ttl.
    /// If ttl is set to "", the associated resource never expires.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'.
    /// Example - "3.5s".
    #[builder(into)]
    #[serde(rename = "ttl")]
    pub r#ttl: Box<String>,
}
