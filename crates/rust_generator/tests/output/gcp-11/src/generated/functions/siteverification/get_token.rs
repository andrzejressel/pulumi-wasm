#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_token {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTokenArgs {
        /// The site identifier. If the type is set to SITE, the identifier is a URL. If the type is
        /// set to INET_DOMAIN, the identifier is a domain name.
        #[builder(into)]
        pub identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of resource to be verified, either a domain or a web site.
        /// Possible values are: `INET_DOMAIN`, `SITE`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The verification method for the Site Verification system to use to verify
        /// this site or domain.
        /// Possible values are: `ANALYTICS`, `DNS_CNAME`, `DNS_TXT`, `FILE`, `META`, `TAG_MANAGER`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub verification_method: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTokenResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub identifier: pulumi_gestalt_rust::Output<String>,
        /// The generated token for use in subsequent verification steps.
        pub token: pulumi_gestalt_rust::Output<String>,
        pub type_: pulumi_gestalt_rust::Output<String>,
        pub verification_method: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetTokenArgs,
    ) -> GetTokenResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let identifier_binding = args.identifier.get_output(context);
        let type__binding = args.type_.get_output(context);
        let verification_method_binding = args.verification_method.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:siteverification/getToken:getToken".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identifier".into(),
                    value: identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "verificationMethod".into(),
                    value: verification_method_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetTokenResult {
            id: o.get_field("id"),
            identifier: o.get_field("identifier"),
            token: o.get_field("token"),
            type_: o.get_field("type"),
            verification_method: o.get_field("verificationMethod"),
        }
    }
}
