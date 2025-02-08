#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableAclAccessPolicy {
    /// The ISO8061 UTC time at which this Access Policy should be valid until.
    #[builder(into)]
    #[serde(rename = "expiry")]
    pub r#expiry: Box<String>,
    /// The permissions which should associated with this Shared Identifier.
    #[builder(into)]
    #[serde(rename = "permissions")]
    pub r#permissions: Box<String>,
    /// The ISO8061 UTC time at which this Access Policy should be valid from.
    #[builder(into)]
    #[serde(rename = "start")]
    pub r#start: Box<String>,
}
