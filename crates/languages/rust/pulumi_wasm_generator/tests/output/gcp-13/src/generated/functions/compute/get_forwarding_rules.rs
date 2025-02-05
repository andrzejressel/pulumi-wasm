pub mod get_forwarding_rules {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetForwardingRulesArgs {
        /// The name of the project.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The region you want to get the forwarding rules from.
        ///
        /// These arguments must be set in either the provider or the resource in order for the information to be queried.
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetForwardingRulesResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The project name being queried.
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// The region being queried.
        pub region: pulumi_wasm_rust::Output<Option<String>>,
        /// This is a list of the forwarding rules in the project. Each forwarding rule will list the backend, description, ip address. name, network, self link, service label, service name, and subnet.
        pub rules: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetForwardingRulesRule>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetForwardingRulesArgs,
    ) -> GetForwardingRulesResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getForwardingRules:getForwardingRules".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetForwardingRulesResult {
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            rules: pulumi_wasm_rust::__private::into_domain(o.extract_field("rules")),
        }
    }
}
