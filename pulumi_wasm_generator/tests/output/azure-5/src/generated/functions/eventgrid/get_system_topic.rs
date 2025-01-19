pub mod get_system_topic {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSystemTopicArgs {
        /// The name of the EventGrid System Topic resource.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which the EventGrid System Topic exists.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetSystemTopicResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below, which contains the Managed Service Identity information for this Event Grid System Topic.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::eventgrid::GetSystemTopicIdentity>,
        >,
        pub location: pulumi_wasm_rust::Output<String>,
        /// The Metric ARM Resource ID of the Event Grid System Topic.
        pub metric_arm_resource_id: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The ID of the Event Grid System Topic ARM Source.
        pub source_arm_resource_id: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which are assigned to the Event Grid System Topic.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// The Topic Type of the Event Grid System Topic.
        pub topic_type: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSystemTopicArgs) -> GetSystemTopicResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:eventgrid/getSystemTopic:getSystemTopic".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "metricArmResourceId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "sourceArmResourceId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "topicType".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSystemTopicResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            metric_arm_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("metricArmResourceId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            source_arm_resource_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceArmResourceId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            topic_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("topicType").unwrap(),
            ),
        }
    }
}
