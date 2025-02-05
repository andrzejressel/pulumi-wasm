pub mod get_domain_identity {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDomainIdentityArgs {
        /// Name of the domain
        #[builder(into)]
        pub domain: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDomainIdentityResult {
        /// ARN of the domain identity.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Name of the domain
        pub domain: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Code which when added to the domain as a TXT record will signal to SES that the owner of the domain has authorized SES to act on their behalf.
        pub verification_token: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetDomainIdentityArgs,
    ) -> GetDomainIdentityResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let domain_binding = args.domain.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ses/getDomainIdentity:getDomainIdentity".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "domain".into(),
                    value: &domain_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDomainIdentityResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            domain: pulumi_wasm_rust::__private::into_domain(o.extract_field("domain")),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            verification_token: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("verificationToken"),
            ),
        }
    }
}
