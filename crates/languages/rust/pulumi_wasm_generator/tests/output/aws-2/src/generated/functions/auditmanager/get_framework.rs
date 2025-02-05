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
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetFrameworkResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            compliance_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("complianceType"),
            ),
            control_sets: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("controlSets"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            framework_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("frameworkType"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
