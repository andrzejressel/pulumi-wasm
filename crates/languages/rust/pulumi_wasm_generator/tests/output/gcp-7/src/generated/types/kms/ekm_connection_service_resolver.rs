#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EkmConnectionServiceResolver {
    /// Optional. The filter applied to the endpoints of the resolved service. If no filter is specified, all endpoints will be considered. An endpoint will be chosen arbitrarily from the filtered list for each request. For endpoint filter syntax and examples, see https://cloud.google.com/service-directory/docs/reference/rpc/google.cloud.servicedirectory.v1#resolveservicerequest.
    #[builder(into, default)]
    #[serde(rename = "endpointFilter")]
    pub r#endpoint_filter: Box<Option<String>>,
    /// Required. The hostname of the EKM replica used at TLS and HTTP layers.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: Box<String>,
    /// Required. A list of leaf server certificates used to authenticate HTTPS connections to the EKM replica. Currently, a maximum of 10 Certificate is supported.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "serverCertificates")]
    pub r#server_certificates: Box<Vec<super::super::types::kms::EkmConnectionServiceResolverServerCertificate>>,
    /// Required. The resource name of the Service Directory service pointing to an EKM replica, in the format projects/*/locations/*/namespaces/*/services/*
    #[builder(into)]
    #[serde(rename = "serviceDirectoryService")]
    pub r#service_directory_service: Box<String>,
}
