pub mod get_control {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetControlArgs {
        #[builder(into, default)]
        pub control_mapping_sources: pulumi_wasm_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::auditmanager::GetControlControlMappingSource,
                >,
            >,
        >,
        /// Name of the control.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Type of control. Valid values are `Custom` and `Standard`.
        #[builder(into)]
        pub type_: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetControlResult {
        pub action_plan_instructions: pulumi_wasm_rust::Output<String>,
        pub action_plan_title: pulumi_wasm_rust::Output<String>,
        pub arn: pulumi_wasm_rust::Output<String>,
        pub control_mapping_sources: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::auditmanager::GetControlControlMappingSource,
                >,
            >,
        >,
        pub description: pulumi_wasm_rust::Output<String>,
        pub id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub testing_information: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetControlArgs,
    ) -> GetControlResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let control_mapping_sources_binding = args
            .control_mapping_sources
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let type__binding = args.type_.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:auditmanager/getControl:getControl".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "controlMappingSources".into(),
                    value: &control_mapping_sources_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "type".into(),
                    value: &type__binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "actionPlanInstructions".into(),
                },
                register_interface::ResultField {
                    name: "actionPlanTitle".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "controlMappingSources".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
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
                register_interface::ResultField {
                    name: "testingInformation".into(),
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
        GetControlResult {
            action_plan_instructions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actionPlanInstructions").unwrap(),
            ),
            action_plan_title: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actionPlanTitle").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            control_mapping_sources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("controlMappingSources").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            testing_information: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("testingInformation").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
