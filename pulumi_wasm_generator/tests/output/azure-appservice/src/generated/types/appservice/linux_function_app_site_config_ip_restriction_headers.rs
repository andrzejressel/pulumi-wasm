#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinuxFunctionAppSiteConfigIpRestrictionHeaders {
    /// Specifies a list of Azure Front Door IDs.
    #[builder(into, default)]
    #[serde(rename = "xAzureFdids")]
    pub r#x_azure_fdids: Box<Option<Vec<String>>>,
    /// Specifies if a Front Door Health Probe should be expected. The only possible value is `1`.
    #[builder(into, default)]
    #[serde(rename = "xFdHealthProbe")]
    pub r#x_fd_health_probe: Box<Option<String>>,
    /// Specifies a list of addresses for which matching should be applied. Omitting this value means allow any.
    #[builder(into, default)]
    #[serde(rename = "xForwardedFors")]
    pub r#x_forwarded_fors: Box<Option<Vec<String>>>,
    /// Specifies a list of Hosts for which matching should be applied.
    #[builder(into, default)]
    #[serde(rename = "xForwardedHosts")]
    pub r#x_forwarded_hosts: Box<Option<Vec<String>>>,
}