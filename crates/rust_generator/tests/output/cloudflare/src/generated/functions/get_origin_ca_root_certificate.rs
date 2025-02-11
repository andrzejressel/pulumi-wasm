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
        context: &pulumi_gestalt_rust::Context,
        args: GetOriginCaRootCertificateArgs,
    ) -> GetOriginCaRootCertificateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let algorithm_binding = args.algorithm.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "cloudflare:index/getOriginCaRootCertificate:getOriginCaRootCertificate"
                .into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "algorithm".into(),
                    value: &algorithm_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetOriginCaRootCertificateResult {
            algorithm: o.get_field("algorithm"),
            cert_pem: o.get_field("certPem"),
            id: o.get_field("id"),
        }
    }
}
