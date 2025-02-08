#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GlobalForwardingRuleServiceDirectoryRegistrations {
    /// Service Directory namespace to register the forwarding rule under.
    #[builder(into, default)]
    #[serde(rename = "namespace")]
    pub r#namespace: Box<Option<String>>,
    /// [Optional] Service Directory region to register this global forwarding rule under.
    /// Default to "us-central1". Only used for PSC for Google APIs. All PSC for
    /// Google APIs Forwarding Rules on the same network should use the same Service
    /// Directory region.
    #[builder(into, default)]
    #[serde(rename = "serviceDirectoryRegion")]
    pub r#service_directory_region: Box<Option<String>>,
}
