#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceNetworkConfigurationIngressConfiguration {
    /// Specifies whether your App Runner service is publicly accessible. To make the service publicly accessible set it to True. To make the service privately accessible, from only within an Amazon VPC set it to False.
    #[builder(into, default)]
    #[serde(rename = "isPubliclyAccessible")]
    pub r#is_publicly_accessible: Box<Option<bool>>,
}