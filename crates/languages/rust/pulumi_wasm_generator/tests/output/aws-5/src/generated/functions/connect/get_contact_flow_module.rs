pub mod get_contact_flow_module {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetContactFlowModuleArgs {
        /// Returns information on a specific Contact Flow Module by contact flow module id
        #[builder(into, default)]
        pub contact_flow_module_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Reference to the hosting Amazon Connect Instance
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Returns information on a specific Contact Flow Module by name
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to the Contact Flow Module.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetContactFlowModuleResult {
        /// ARN of the Contact Flow Module.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub contact_flow_module_id: pulumi_wasm_rust::Output<String>,
        /// Logic of the Contact Flow Module.
        pub content: pulumi_wasm_rust::Output<String>,
        /// Description of the Contact Flow Module.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        /// Type of Contact Flow Module Module. Values are either `ACTIVE` or `ARCHIVED`.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Status of the Contact Flow Module Module. Values are either `PUBLISHED` or `SAVED`.
        pub status: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the Contact Flow Module.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetContactFlowModuleArgs,
    ) -> GetContactFlowModuleResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let contact_flow_module_id_binding = args
            .contact_flow_module_id
            .get_output(context)
            .get_inner();
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:connect/getContactFlowModule:getContactFlowModule".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "contactFlowModuleId".into(),
                    value: &contact_flow_module_id_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetContactFlowModuleResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            contact_flow_module_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("contactFlowModuleId"),
            ),
            content: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("content"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            status: pulumi_wasm_rust::__private::into_domain(o.extract_field("status")),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
