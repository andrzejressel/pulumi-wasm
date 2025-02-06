#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StandardSiteConfigIpRestrictionHeaders {
    /// A list of allowed Azure FrontDoor IDs in UUID notation with a maximum of 8.
    #[builder(into, default)]
    #[serde(rename = "xAzureFdids")]
    pub r#x_azure_fdids: Box<Option<Vec<String>>>,
    /// A list to allow the Azure FrontDoor health probe header. Only allowed value is `1`.
    #[builder(into, default)]
    #[serde(rename = "xFdHealthProbe")]
    pub r#x_fd_health_probe: Box<Option<String>>,
    /// A list of allowed 'X-Forwarded-For' IPs in CIDR notation with a maximum of 8.
    #[builder(into, default)]
    #[serde(rename = "xForwardedFors")]
    pub r#x_forwarded_fors: Box<Option<Vec<String>>>,
    /// A list of allowed 'X-Forwarded-Host' domains with a maximum of 8.
    #[builder(into, default)]
    #[serde(rename = "xForwardedHosts")]
    pub r#x_forwarded_hosts: Box<Option<Vec<String>>>,
}
