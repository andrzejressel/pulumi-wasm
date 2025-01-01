#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FrontdoorSecretSecret {
    /// A `customer_certificate` block as defined below. Changing this forces a new Front Door Secret to be created.
    #[builder(into)]
    #[serde(rename = "customerCertificates")]
    pub r#customer_certificates: Box<Vec<super::super::types::cdn::FrontdoorSecretSecretCustomerCertificate>>,
}
