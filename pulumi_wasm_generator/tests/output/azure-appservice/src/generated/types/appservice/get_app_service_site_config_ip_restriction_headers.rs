#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAppServiceSiteConfigIpRestrictionHeaders {
    #[builder(into)]
    #[serde(rename = "xAzureFdids")]
    pub r#x_azure_fdids: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "xFdHealthProbes")]
    pub r#x_fd_health_probes: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "xForwardedFors")]
    pub r#x_forwarded_fors: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "xForwardedHosts")]
    pub r#x_forwarded_hosts: Box<Vec<String>>,
}
