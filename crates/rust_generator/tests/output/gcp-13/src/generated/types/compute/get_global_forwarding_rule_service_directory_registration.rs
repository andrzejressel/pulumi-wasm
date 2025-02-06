#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetGlobalForwardingRuleServiceDirectoryRegistration {
    /// Service Directory namespace to register the forwarding rule under.
    #[builder(into)]
    #[serde(rename = "namespace")]
    pub r#namespace: Box<String>,
    /// [Optional] Service Directory region to register this global forwarding rule under.
    /// Default to "us-central1". Only used for PSC for Google APIs. All PSC for
    /// Google APIs Forwarding Rules on the same network should use the same Service
    /// Directory region.
    #[builder(into)]
    #[serde(rename = "serviceDirectoryRegion")]
    pub r#service_directory_region: Box<String>,
}
