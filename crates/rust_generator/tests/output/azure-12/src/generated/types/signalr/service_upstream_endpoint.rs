#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceUpstreamEndpoint {
    /// The categories to match on, or `*` for all.
    #[builder(into)]
    #[serde(rename = "categoryPatterns")]
    pub r#category_patterns: Box<Vec<String>>,
    /// The events to match on, or `*` for all.
    #[builder(into)]
    #[serde(rename = "eventPatterns")]
    pub r#event_patterns: Box<Vec<String>>,
    /// The hubs to match on, or `*` for all.
    #[builder(into)]
    #[serde(rename = "hubPatterns")]
    pub r#hub_patterns: Box<Vec<String>>,
    /// The upstream URL Template. This can be a url or a template such as `http://host.com/{hub}/api/{category}/{event}`.
    #[builder(into)]
    #[serde(rename = "urlTemplate")]
    pub r#url_template: Box<String>,
    /// Specifies the Managed Identity IDs to be assigned to this signalR upstream setting by using resource uuid as both system assigned and user assigned identity is supported.
    #[builder(into, default)]
    #[serde(rename = "userAssignedIdentityId")]
    pub r#user_assigned_identity_id: Box<Option<String>>,
}
