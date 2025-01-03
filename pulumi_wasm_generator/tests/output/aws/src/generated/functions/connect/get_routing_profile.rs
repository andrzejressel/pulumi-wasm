pub mod get_routing_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRoutingProfileArgs {
        /// Reference to the hosting Amazon Connect Instance
        #[builder(into)]
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// Returns information on a specific Routing Profile by name
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Returns information on a specific Routing Profile by Routing Profile id
        #[builder(into, default)]
        pub routing_profile_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to the Routing Profile.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetRoutingProfileResult {
        /// ARN of the Routing Profile.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Specifies the default outbound queue for the Routing Profile.
        pub default_outbound_queue_id: pulumi_wasm_rust::Output<String>,
        /// Description of the Routing Profile.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// One or more `media_concurrencies` blocks that specify the channels that agents can handle in the Contact Control Panel (CCP) for this Routing Profile. The `media_concurrencies` block is documented below.
        pub media_concurrencies: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::connect::GetRoutingProfileMediaConcurrency>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        /// One or more `queue_configs` blocks that specify the inbound queues associated with the routing profile. If no queue is added, the agent only can make outbound calls. The `queue_configs` block is documented below.
        pub queue_configs: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::connect::GetRoutingProfileQueueConfig>,
        >,
        pub routing_profile_id: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to the Routing Profile.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetRoutingProfileArgs) -> GetRoutingProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let instance_id_binding = args.instance_id.get_inner();
        let name_binding = args.name.get_inner();
        let routing_profile_id_binding = args.routing_profile_id.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:connect/getRoutingProfile:getRoutingProfile".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "routingProfileId".into(),
                    value: &routing_profile_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "defaultOutboundQueueId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceId".into(),
                },
                register_interface::ResultField {
                    name: "mediaConcurrencies".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "queueConfigs".into(),
                },
                register_interface::ResultField {
                    name: "routingProfileId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRoutingProfileResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            default_outbound_queue_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultOutboundQueueId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
            media_concurrencies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mediaConcurrencies").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            queue_configs: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("queueConfigs").unwrap(),
            ),
            routing_profile_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routingProfileId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
        }
    }
}
