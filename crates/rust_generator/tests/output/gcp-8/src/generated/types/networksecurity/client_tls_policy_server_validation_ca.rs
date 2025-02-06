#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClientTlsPolicyServerValidationCa {
    /// The certificate provider instance specification that will be passed to the data plane, which will be used to load necessary credential information.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "certificateProviderInstance")]
    pub r#certificate_provider_instance: Box<Option<super::super::types::networksecurity::ClientTlsPolicyServerValidationCaCertificateProviderInstance>>,
    /// gRPC specific configuration to access the gRPC server to obtain the cert and private key.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "grpcEndpoint")]
    pub r#grpc_endpoint: Box<Option<super::super::types::networksecurity::ClientTlsPolicyServerValidationCaGrpcEndpoint>>,
}
