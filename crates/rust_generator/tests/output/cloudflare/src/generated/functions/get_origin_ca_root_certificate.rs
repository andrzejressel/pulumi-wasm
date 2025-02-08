#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_origin_ca_root_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOriginCaRootCertificateArgs {
        /// The name of the algorithm used when creating an Origin CA certificate. Available values: `rsa`, `ecc`.
        #[builder(into)]
        pub algorithm: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetOriginCaRootCertificateResult {
        /// The name of the algorithm used when creating an Origin CA certificate. Available values: `rsa`, `ecc`.
        pub algorithm: pulumi_gestalt_rust::Output<String>,
        /// The Origin CA root certificate in PEM format.
        pub cert_pem: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetOriginCaRootCertificateArgs,
    ) -> GetOriginCaRootCertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let algorithm_binding = args.algorithm.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "cloudflare:index/getOriginCaRootCertificate:getOriginCaRootCertificate"
                .into(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "algorithm".into(),
                    value: &algorithm_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetOriginCaRootCertificateResult {
            algorithm: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("algorithm"),
            ),
            cert_pem: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certPem"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}
