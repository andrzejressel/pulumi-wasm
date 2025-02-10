/// Creates an HSM client certificate that an Amazon Redshift cluster will use to connect to the client's HSM in order to store and retrieve the keys used to encrypt the cluster databases.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = hsm_client_certificate::create(
///         "example",
///         HsmClientCertificateArgs::builder()
///             .hsm_client_certificate_identifier("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Redshift HSM Client Certificates using `hsm_client_certificate_identifier`. For example:
///
/// ```sh
/// $ pulumi import aws:redshift/hsmClientCertificate:HsmClientCertificate test example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod hsm_client_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HsmClientCertificateArgs {
        /// The identifier of the HSM client certificate.
        #[builder(into)]
        pub hsm_client_certificate_identifier: pulumi_gestalt_rust::InputOrOutput<
            String,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct HsmClientCertificateResult {
        /// Amazon Resource Name (ARN) of the Hsm Client Certificate.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The identifier of the HSM client certificate.
        pub hsm_client_certificate_identifier: pulumi_gestalt_rust::Output<String>,
        /// The public key that the Amazon Redshift cluster will use to connect to the HSM. You must register the public key in the HSM.
        pub hsm_client_certificate_public_key: pulumi_gestalt_rust::Output<String>,
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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HsmClientCertificateArgs,
    ) -> HsmClientCertificateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let hsm_client_certificate_identifier_binding = args
            .hsm_client_certificate_identifier
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:redshift/hsmClientCertificate:HsmClientCertificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hsmClientCertificateIdentifier".into(),
                    value: hsm_client_certificate_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        HsmClientCertificateResult {
            arn: o.get_field("arn"),
            hsm_client_certificate_identifier: o
                .get_field("hsmClientCertificateIdentifier"),
            hsm_client_certificate_public_key: o
                .get_field("hsmClientCertificatePublicKey"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
