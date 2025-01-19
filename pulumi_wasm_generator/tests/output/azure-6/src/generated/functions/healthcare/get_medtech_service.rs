pub mod get_medtech_service {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetMedtechServiceArgs {
        /// The name of the Healthcare Med Tech Service.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The id of the Healthcare Workspace in which the Healthcare Med Tech Service exists.
        #[builder(into)]
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetMedtechServiceResult {
        /// The Device Mappings of the Med Tech Service.
        pub device_mapping_json: pulumi_wasm_rust::Output<String>,
        /// The Consumer Group of the Event Hub of the Healthcare Med Tech Service.
        pub eventhub_consumer_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Event Hub of the Healthcare Med Tech Service.
        pub eventhub_name: pulumi_wasm_rust::Output<String>,
        /// The namespace name of the Event Hub of the Healthcare Med Tech Service.
        pub eventhub_namespace_name: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The `identity` block as defined below.
        pub identities: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::healthcare::GetMedtechServiceIdentity>,
        >,
        pub name: pulumi_wasm_rust::Output<String>,
        pub workspace_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetMedtechServiceArgs) -> GetMedtechServiceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_inner();
        let workspace_id_binding = args.workspace_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:healthcare/getMedtechService:getMedtechService".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "deviceMappingJson".into(),
                },
                register_interface::ResultField {
                    name: "eventhubConsumerGroupName".into(),
                },
                register_interface::ResultField {
                    name: "eventhubName".into(),
                },
                register_interface::ResultField {
                    name: "eventhubNamespaceName".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "identities".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "workspaceId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetMedtechServiceResult {
            device_mapping_json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deviceMappingJson").unwrap(),
            ),
            eventhub_consumer_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventhubConsumerGroupName").unwrap(),
            ),
            eventhub_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventhubName").unwrap(),
            ),
            eventhub_namespace_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventhubNamespaceName").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            identities: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identities").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            workspace_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("workspaceId").unwrap(),
            ),
        }
    }
}
