#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCertificateArgs {
        /// A customer-assigned name for the certificate. Identifiers must begin with a letter and must contain only ASCII letters, digits, and hyphens. They can't end with a hyphen or contain two consecutive hyphens.
        #[builder(into)]
        pub certificate_id: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetCertificateResult {
        /// The Amazon Resource Name (ARN) for the certificate.
        pub certificate_arn: pulumi_gestalt_rust::Output<String>,
        /// The date that the certificate was created.
        pub certificate_creation_date: pulumi_gestalt_rust::Output<String>,
        pub certificate_id: pulumi_gestalt_rust::Output<String>,
        /// The owner of the certificate.
        pub certificate_owner: pulumi_gestalt_rust::Output<String>,
        /// The contents of a .pem file, which contains an X.509 certificate.
        pub certificate_pem: pulumi_gestalt_rust::Output<String>,
        /// The owner of the certificate.
        pub certificate_wallet: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The key length of the cryptographic algorithm being used.
        pub key_length: pulumi_gestalt_rust::Output<i32>,
        /// The algorithm for the certificate.
        pub signing_algorithm: pulumi_gestalt_rust::Output<String>,
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The beginning date that the certificate is valid.
        pub valid_from_date: pulumi_gestalt_rust::Output<String>,
        /// The final date that the certificate is valid.
        pub valid_to_date: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCertificateArgs,
    ) -> GetCertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let certificate_id_binding = args.certificate_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:dms/getCertificate:getCertificate".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateId".into(),
                    value: certificate_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCertificateResult {
            certificate_arn: o.get_field("certificateArn"),
            certificate_creation_date: o.get_field("certificateCreationDate"),
            certificate_id: o.get_field("certificateId"),
            certificate_owner: o.get_field("certificateOwner"),
            certificate_pem: o.get_field("certificatePem"),
            certificate_wallet: o.get_field("certificateWallet"),
            id: o.get_field("id"),
            key_length: o.get_field("keyLength"),
            signing_algorithm: o.get_field("signingAlgorithm"),
            tags: o.get_field("tags"),
            valid_from_date: o.get_field("validFromDate"),
            valid_to_date: o.get_field("validToDate"),
        }
    }
}
