#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetWindowsWebAppSiteConfigScmIpRestrictionHeader {
    /// The list of Azure Front Door IDs.
    #[builder(into)]
    #[serde(rename = "xAzureFdids")]
    pub r#x_azure_fdids: Box<Vec<String>>,
    /// Specifies if a Front Door Health Probe is expected.
    #[builder(into)]
    #[serde(rename = "xFdHealthProbes")]
    pub r#x_fd_health_probes: Box<Vec<String>>,
    /// The list of addresses for which matching is applied.
    #[builder(into)]
    #[serde(rename = "xForwardedFors")]
    pub r#x_forwarded_fors: Box<Vec<String>>,
    /// The list of Hosts for which matching will be applied.
    #[builder(into)]
    #[serde(rename = "xForwardedHosts")]
    pub r#x_forwarded_hosts: Box<Vec<String>>,
}
