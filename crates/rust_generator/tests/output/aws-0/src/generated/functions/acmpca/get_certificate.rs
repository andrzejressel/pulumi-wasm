#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetCertificateArgs {
        /// ARN of the certificate issued by the private certificate authority.
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN of the certificate authority.
        #[builder(into)]
        pub certificate_authority_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetCertificateResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// PEM-encoded certificate value.
        pub certificate: pulumi_gestalt_rust::Output<String>,
        pub certificate_authority_arn: pulumi_gestalt_rust::Output<String>,
        /// PEM-encoded certificate chain that includes any intermediate certificates and chains up to root CA.
        pub certificate_chain: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetCertificateArgs,
    ) -> GetCertificateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let certificate_authority_arn_binding = args
            .certificate_authority_arn
            .get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:acmpca/getCertificate:getCertificate".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateAuthorityArn".into(),
                    value: certificate_authority_arn_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetCertificateResult {
            arn: o.get_field("arn"),
            certificate: o.get_field("certificate"),
            certificate_authority_arn: o.get_field("certificateAuthorityArn"),
            certificate_chain: o.get_field("certificateChain"),
            id: o.get_field("id"),
        }
    }
}
