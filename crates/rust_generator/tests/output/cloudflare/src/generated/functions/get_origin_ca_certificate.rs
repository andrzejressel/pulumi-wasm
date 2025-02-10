#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_origin_ca_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetOriginCaCertificateArgs {
        /// The Origin CA Certificate unique identifier.
        #[builder(into)]
        pub id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetOriginCaCertificateResult {
        /// The Origin CA certificate.
        pub certificate: pulumi_gestalt_rust::Output<String>,
        /// The timestamp when the certificate will expire.
        pub expires_on: pulumi_gestalt_rust::Output<String>,
        /// A list of hostnames or wildcard names bound to the certificate.
        pub hostnames: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The Origin CA Certificate unique identifier.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The signature type desired on the certificate. Available values: `origin-rsa`, `origin-ecc`, `keyless-certificate`
        pub request_type: pulumi_gestalt_rust::Output<String>,
        /// The timestamp when the certificate was revoked.
        pub revoked_at: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetOriginCaCertificateArgs,
    ) -> GetOriginCaCertificateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let id_binding = args.id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "cloudflare:index/getOriginCaCertificate:getOriginCaCertificate"
                .into(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetOriginCaCertificateResult {
            certificate: o.get_field("certificate"),
            expires_on: o.get_field("expiresOn"),
            hostnames: o.get_field("hostnames"),
            id: o.get_field("id"),
            request_type: o.get_field("requestType"),
            revoked_at: o.get_field("revokedAt"),
        }
    }
}
