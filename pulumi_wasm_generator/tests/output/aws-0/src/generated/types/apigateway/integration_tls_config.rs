#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IntegrationTlsConfig {
    /// Whether or not API Gateway skips verification that the certificate for an integration endpoint is issued by a [supported certificate authority](https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-supported-certificate-authorities-for-http-endpoints.html). This isn’t recommended, but it enables you to use certificates that are signed by private certificate authorities, or certificates that are self-signed. If enabled, API Gateway still performs basic certificate validation, which includes checking the certificate's expiration date, hostname, and presence of a root certificate authority. Supported only for `HTTP` and `HTTP_PROXY` integrations.
    #[builder(into, default)]
    #[serde(rename = "insecureSkipVerification")]
    pub r#insecure_skip_verification: Box<Option<bool>>,
}
