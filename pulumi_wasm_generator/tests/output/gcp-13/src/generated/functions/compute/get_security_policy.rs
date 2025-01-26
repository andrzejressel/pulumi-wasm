pub mod get_security_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSecurityPolicyArgs {
        /// The name of the security policy. Provide either this or a `self_link`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The project in which the resource belongs. If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The self_link of the security policy. Provide either this or a `name`
        #[builder(into, default)]
        pub self_link: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSecurityPolicyResult {
        pub adaptive_protection_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetSecurityPolicyAdaptiveProtectionConfig,
            >,
        >,
        pub advanced_options_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetSecurityPolicyAdvancedOptionsConfig,
            >,
        >,
        pub description: pulumi_wasm_rust::Output<String>,
        pub fingerprint: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub recaptcha_options_configs: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::compute::GetSecurityPolicyRecaptchaOptionsConfig,
            >,
        >,
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetSecurityPolicyRule>,
        >,
        pub self_link: pulumi_wasm_rust::Output<Option<String>>,
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetSecurityPolicyArgs,
    ) -> GetSecurityPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let self_link_binding = args.self_link.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getSecurityPolicy:getSecurityPolicy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "selfLink".into(),
                    value: &self_link_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "adaptiveProtectionConfigs".into(),
                },
                register_interface::ResultField {
                    name: "advancedOptionsConfigs".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "fingerprint".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "recaptchaOptionsConfigs".into(),
                },
                register_interface::ResultField {
                    name: "rules".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSecurityPolicyResult {
            adaptive_protection_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("adaptiveProtectionConfigs").unwrap(),
            ),
            advanced_options_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("advancedOptionsConfigs").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fingerprint").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            recaptcha_options_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recaptchaOptionsConfigs").unwrap(),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rules").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
