pub mod get_external_access_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetExternalAccessRuleArgs {
        /// Name of the resource.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The resource name of the network policy that this cluster belongs.
        #[builder(into)]
        pub parent: pulumi_wasm_rust::Output<String>,
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
    pub fn invoke(args: GetExternalAccessRuleArgs) -> GetExternalAccessRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let parent_binding = args.parent.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:vmwareengine/getExternalAccessRule:getExternalAccessRule".into(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "action".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "destinationIpRanges".into(),
                },
                register_interface::ResultField {
                    name: "destinationPorts".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ipProtocol".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parent".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
                register_interface::ResultField {
                    name: "sourceIpRanges".into(),
                },
                register_interface::ResultField {
                    name: "sourcePorts".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetExternalAccessRuleResult {
            action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("action").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            destination_ip_ranges: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationIpRanges").unwrap(),
            ),
            destination_ports: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationPorts").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ip_protocol: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipProtocol").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parent").unwrap(),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
            source_ip_ranges: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceIpRanges").unwrap(),
            ),
            source_ports: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourcePorts").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
