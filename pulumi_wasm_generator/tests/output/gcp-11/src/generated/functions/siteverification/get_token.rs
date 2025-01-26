pub mod get_token {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetTokenArgs {
        /// The site identifier. If the type is set to SITE, the identifier is a URL. If the type is
        /// set to INET_DOMAIN, the identifier is a domain name.
        #[builder(into)]
        pub identifier: pulumi_wasm_rust::InputOrOutput<String>,
        /// The type of resource to be verified, either a domain or a web site.
        /// Possible values are: `INET_DOMAIN`, `SITE`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
        /// The verification method for the Site Verification system to use to verify
        /// this site or domain.
        /// Possible values are: `ANALYTICS`, `DNS_CNAME`, `DNS_TXT`, `FILE`, `META`, `TAG_MANAGER`.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub verification_method: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetTokenResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub identifier: pulumi_wasm_rust::Output<String>,
        /// The generated token for use in subsequent verification steps.
        pub token: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
        pub verification_method: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetTokenArgs,
    ) -> GetTokenResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let identifier_binding = args.identifier.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let verification_method_binding = args
            .verification_method
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:siteverification/getToken:getToken".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "identifier".into(),
                    value: &identifier_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
                register_interface::ObjectField {
                    name: "verificationMethod".into(),
                    value: &verification_method_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetTokenResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identifier"),
            ),
            token: pulumi_wasm_rust::__private::into_domain(o.extract_field("token")),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            verification_method: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("verificationMethod"),
            ),
        }
    }
}
