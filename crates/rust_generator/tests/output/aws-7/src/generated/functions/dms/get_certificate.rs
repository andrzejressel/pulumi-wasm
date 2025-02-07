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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetCertificateArgs,
    ) -> GetCertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let certificate_id_binding = args.certificate_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:dms/getCertificate:getCertificate".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateId".into(),
                    value: &certificate_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetCertificateResult {
            certificate_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateArn"),
            ),
            certificate_creation_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateCreationDate"),
            ),
            certificate_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateId"),
            ),
            certificate_owner: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateOwner"),
            ),
            certificate_pem: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificatePem"),
            ),
            certificate_wallet: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateWallet"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            key_length: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("keyLength"),
            ),
            signing_algorithm: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("signingAlgorithm"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            valid_from_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("validFromDate"),
            ),
            valid_to_date: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("validToDate"),
            ),
        }
    }
}
