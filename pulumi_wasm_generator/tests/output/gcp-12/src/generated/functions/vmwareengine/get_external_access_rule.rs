pub mod get_external_access_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetExternalAccessRuleArgs {
        /// Name of the resource.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The resource name of the network policy that this cluster belongs.
        #[builder(into)]
        pub parent: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetExternalAccessRuleResult {
        pub action: pulumi_wasm_rust::Output<String>,
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub destination_ip_ranges: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::vmwareengine::GetExternalAccessRuleDestinationIpRange,
            >,
        >,
        pub destination_ports: pulumi_wasm_rust::Output<Vec<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub ip_protocol: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub parent: pulumi_wasm_rust::Output<String>,
        pub priority: pulumi_wasm_rust::Output<i32>,
        pub source_ip_ranges: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::vmwareengine::GetExternalAccessRuleSourceIpRange,
            >,
        >,
        pub source_ports: pulumi_wasm_rust::Output<Vec<String>>,
        pub state: pulumi_wasm_rust::Output<String>,
        pub uid: pulumi_wasm_rust::Output<String>,
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetExternalAccessRuleArgs,
    ) -> GetExternalAccessRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let parent_binding = args.parent.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:vmwareengine/getExternalAccessRule:getExternalAccessRule".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parent".into(),
                    value: &parent_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetExternalAccessRuleResult {
            action: pulumi_wasm_rust::__private::into_domain(o.extract_field("action")),
            create_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("createTime"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            destination_ip_ranges: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("destinationIpRanges"),
            ),
            destination_ports: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("destinationPorts"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            ip_protocol: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipProtocol"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            parent: pulumi_wasm_rust::__private::into_domain(o.extract_field("parent")),
            priority: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
            source_ip_ranges: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourceIpRanges"),
            ),
            source_ports: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("sourcePorts"),
            ),
            state: pulumi_wasm_rust::__private::into_domain(o.extract_field("state")),
            uid: pulumi_wasm_rust::__private::into_domain(o.extract_field("uid")),
            update_time: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("updateTime"),
            ),
        }
    }
}
