pub mod get_serverless_security_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetServerlessSecurityPolicyArgs {
        /// Name of the policy
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Type of security policy. One of `encryption` or `network`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetServerlessSecurityPolicyResult {
        /// The date the security policy was created.
        pub created_date: pulumi_wasm_rust::Output<String>,
        /// Description of the security policy.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The date the security policy was last modified.
        pub last_modified_date: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// The JSON policy document without any whitespaces.
        pub policy: pulumi_wasm_rust::Output<String>,
        /// Version of the policy.
        pub policy_version: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetServerlessSecurityPolicyArgs,
    ) -> GetServerlessSecurityPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:opensearch/getServerlessSecurityPolicy:getServerlessSecurityPolicy"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetServerlessSecurityPolicyResult {
            created_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createdDate"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            last_modified_date: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lastModifiedDate"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            policy: pulumi_wasm_rust::__private::into_domain(o.extract_field("policy")),
            policy_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("policyVersion"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}
