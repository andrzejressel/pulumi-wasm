pub mod get_framework {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFrameworkArgs {
        #[builder(into, default)]
        pub control_sets: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::super::types::auditmanager::GetFrameworkControlSet>>,
        >,
        #[builder(into)]
        pub framework_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the framework.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFrameworkResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        pub compliance_type: pulumi_wasm_rust::Output<String>,
        pub control_sets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::auditmanager::GetFrameworkControlSet>>,
        >,
        pub description: pulumi_wasm_rust::Output<String>,
        pub framework_type: pulumi_wasm_rust::Output<String>,
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetFrameworkArgs,
    ) -> GetFrameworkResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let control_sets_binding = args.control_sets.get_output(context).get_inner();
        let framework_type_binding = args.framework_type.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:auditmanager/getFramework:getFramework".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "controlSets".into(),
                    value: &control_sets_binding,
                },
                register_interface::ObjectField {
                    name: "frameworkType".into(),
                    value: &framework_type_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "complianceType".into(),
                },
                register_interface::ResultField {
                    name: "controlSets".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "frameworkType".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetFrameworkResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            compliance_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("complianceType").unwrap(),
            ),
            control_sets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("controlSets").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            framework_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("frameworkType").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
