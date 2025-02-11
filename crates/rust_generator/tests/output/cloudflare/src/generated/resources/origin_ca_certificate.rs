/// Provides a Cloudflare Origin CA certificate used to protect traffic to your origin without involving a third party Certificate Authority.
///
/// > Since v3.32.0
///    all authentication schemes are supported for managing Origin CA certificates.
///    Versions prior to v3.32.0 will still need to use `api_user_service_key`.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: tls:privateKey
///     properties:
///       algorithm: RSA
///   exampleCertRequest:
///     type: tls:certRequest
///     name: example
///     properties:
///       privateKeyPem: ${example.privateKeyPem}
///       subject:
///         - commonName:
///           organization: Terraform Test
///   exampleOriginCaCertificate:
///     type: cloudflare:OriginCaCertificate
///     name: example
///     properties:
///       csr: ${exampleCertRequest.certRequestPem}
///       hostnames:
///         - example.com
///       requestType: origin-rsa
///       requestedValidity: 7
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/originCaCertificate:OriginCaCertificate example <certificate_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod origin_ca_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OriginCaCertificateArgs {
        /// The Certificate Signing Request. Must be newline-encoded. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub csr: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of hostnames or wildcard names bound to the certificate. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub hostnames: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        #[builder(into, default)]
        pub min_days_for_renewal: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The signature type desired on the certificate. Available values: `origin-rsa`, `origin-ecc`, `keyless-certificate`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub request_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The number of days for which the certificate should be valid. Available values: `7`, `30`, `90`, `365`, `730`, `1095`, `5475`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub requested_validity: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct OriginCaCertificateResult {
        /// The Origin CA certificate.
        pub certificate: pulumi_gestalt_rust::Output<String>,
        /// The Certificate Signing Request. Must be newline-encoded. **Modifying this attribute will force creation of a new resource.**
        pub csr: pulumi_gestalt_rust::Output<String>,
        /// The datetime when the certificate will expire.
        pub expires_on: pulumi_gestalt_rust::Output<String>,
        /// A list of hostnames or wildcard names bound to the certificate. **Modifying this attribute will force creation of a new resource.**
        pub hostnames: pulumi_gestalt_rust::Output<Vec<String>>,
        pub min_days_for_renewal: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The signature type desired on the certificate. Available values: `origin-rsa`, `origin-ecc`, `keyless-certificate`. **Modifying this attribute will force creation of a new resource.**
        pub request_type: pulumi_gestalt_rust::Output<String>,
        /// The number of days for which the certificate should be valid. Available values: `7`, `30`, `90`, `365`, `730`, `1095`, `5475`. **Modifying this attribute will force creation of a new resource.**
        pub requested_validity: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OriginCaCertificateArgs,
    ) -> OriginCaCertificateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let csr_binding = args.csr.get_output(context);
        let hostnames_binding = args.hostnames.get_output(context);
        let min_days_for_renewal_binding = args.min_days_for_renewal.get_output(context);
        let request_type_binding = args.request_type.get_output(context);
        let requested_validity_binding = args.requested_validity.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/originCaCertificate:OriginCaCertificate".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "csr".into(),
                    value: &csr_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostnames".into(),
                    value: &hostnames_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minDaysForRenewal".into(),
                    value: &min_days_for_renewal_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestType".into(),
                    value: &request_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "requestedValidity".into(),
                    value: &requested_validity_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        OriginCaCertificateResult {
            certificate: o.get_field("certificate"),
            csr: o.get_field("csr"),
            expires_on: o.get_field("expiresOn"),
            hostnames: o.get_field("hostnames"),
            min_days_for_renewal: o.get_field("minDaysForRenewal"),
            request_type: o.get_field("requestType"),
            requested_validity: o.get_field("requestedValidity"),
        }
    }
}
