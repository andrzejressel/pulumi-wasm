#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLaunchTemplatePrivateDnsNameOption {
    #[builder(into)]
    #[serde(rename = "enableResourceNameDnsARecord")]
    pub r#enable_resource_name_dns_a_record: Box<bool>,
    #[builder(into)]
    #[serde(rename = "enableResourceNameDnsAaaaRecord")]
    pub r#enable_resource_name_dns_aaaa_record: Box<bool>,
    #[builder(into)]
    #[serde(rename = "hostnameType")]
    pub r#hostname_type: Box<String>,
}
