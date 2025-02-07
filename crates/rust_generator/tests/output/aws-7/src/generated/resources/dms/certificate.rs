/// Provides a DMS (Data Migration Service) certificate resource. DMS certificates can be created, deleted, and imported.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   # Create a new certificate
///   test:
///     type: aws:dms:Certificate
///     properties:
///       certificateId: test-dms-certificate-tf
///       certificatePem: '...'
///       tags:
///         Name: test
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import certificates using the `certificate_id`. For example:
///
/// ```sh
/// $ pulumi import aws:dms/certificate:Certificate test test-dms-certificate-tf
/// ```
pub mod certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// The certificate identifier.
        ///
        /// - Must contain from 1 to 255 alphanumeric characters and hyphens.
        #[builder(into)]
        pub certificate_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The contents of the .pem X.509 certificate file for the certificate. Either `certificate_pem` or `certificate_wallet` must be set.
        #[builder(into, default)]
        pub certificate_pem: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The contents of the Oracle Wallet certificate for use with SSL, provided as a base64-encoded String. Either `certificate_pem` or `certificate_wallet` must be set.
        #[builder(into, default)]
        pub certificate_wallet: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// The Amazon Resource Name (ARN) for the certificate.
        pub certificate_arn: pulumi_gestalt_rust::Output<String>,
        /// The certificate identifier.
        ///
        /// - Must contain from 1 to 255 alphanumeric characters and hyphens.
        pub certificate_id: pulumi_gestalt_rust::Output<String>,
        /// The contents of the .pem X.509 certificate file for the certificate. Either `certificate_pem` or `certificate_wallet` must be set.
        pub certificate_pem: pulumi_gestalt_rust::Output<Option<String>>,
        /// The contents of the Oracle Wallet certificate for use with SSL, provided as a base64-encoded String. Either `certificate_pem` or `certificate_wallet` must be set.
        pub certificate_wallet: pulumi_gestalt_rust::Output<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CertificateArgs,
    ) -> CertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let certificate_id_binding = args.certificate_id.get_output(context).get_inner();
        let certificate_pem_binding = args
            .certificate_pem
            .get_output(context)
            .get_inner();
        let certificate_wallet_binding = args
            .certificate_wallet
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:dms/certificate:Certificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateId".into(),
                    value: &certificate_id_binding,
                },
                register_interface::ObjectField {
                    name: "certificatePem".into(),
                    value: &certificate_pem_binding,
                },
                register_interface::ObjectField {
                    name: "certificateWallet".into(),
                    value: &certificate_wallet_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CertificateResult {
            certificate_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateArn"),
            ),
            certificate_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateId"),
            ),
            certificate_pem: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificatePem"),
            ),
            certificate_wallet: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateWallet"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
