#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServerTlsPolicyMtlsPolicy {
    /// Required if the policy is to be used with Traffic Director. For external HTTPS load balancers it must be empty.
    /// Defines the mechanism to obtain the Certificate Authority certificate to validate the client certificate.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "clientValidationCas")]
    pub r#client_validation_cas: Box<Option<Vec<super::super::types::networksecurity::ServerTlsPolicyMtlsPolicyClientValidationCa>>>,
    /// When the client presents an invalid certificate or no certificate to the load balancer, the clientValidationMode specifies how the client connection is handled.
    /// Required if the policy is to be used with the external HTTPS load balancing. For Traffic Director it must be empty.
    /// Possible values are: `CLIENT_VALIDATION_MODE_UNSPECIFIED`, `ALLOW_INVALID_OR_MISSING_CLIENT_CERT`, `REJECT_INVALID`.
    #[builder(into, default)]
    #[serde(rename = "clientValidationMode")]
    pub r#client_validation_mode: Box<Option<String>>,
    /// Reference to the TrustConfig from certificatemanager.googleapis.com namespace.
    /// If specified, the chain validation will be performed against certificates configured in the given TrustConfig.
    /// Allowed only if the policy is to be used with external HTTPS load balancers.
    #[builder(into, default)]
    #[serde(rename = "clientValidationTrustConfig")]
    pub r#client_validation_trust_config: Box<Option<String>>,
}
