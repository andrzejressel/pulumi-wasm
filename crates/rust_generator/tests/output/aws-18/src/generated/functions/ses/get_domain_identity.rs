#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_domain_identity {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDomainIdentityArgs {
        /// Name of the domain
        #[builder(into)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDomainIdentityResult {
        /// ARN of the domain identity.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the domain
        pub domain: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Code which when added to the domain as a TXT record will signal to SES that the owner of the domain has authorized SES to act on their behalf.
        pub verification_token: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetDomainIdentityArgs,
    ) -> GetDomainIdentityResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let domain_binding = args.domain.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ses/getDomainIdentity:getDomainIdentity".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "domain".into(),
                    value: domain_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetDomainIdentityResult {
            arn: o.get_field("arn"),
            domain: o.get_field("domain"),
            id: o.get_field("id"),
            verification_token: o.get_field("verificationToken"),
        }
    }
}
