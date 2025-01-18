/// Manages an IotHub ServiceBus Queue Endpoint
///
/// > **NOTE:** Endpoints can be defined either directly on the `azure.iot.IoTHub` resource, or using the `azurerm_iothub_endpoint_*` resources - but the two ways of defining the endpoints cannot be used together. If both are used against the same IoTHub, spurious changes will occur. Also, defining a `azurerm_iothub_endpoint_*` resource and another endpoint of a different type directly on the `azure.iot.IoTHub` resource is not supported.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleNamespace:
///     type: azure:servicebus:Namespace
///     name: example
///     properties:
///       name: exampleNamespace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Standard
///   exampleQueue:
///     type: azure:servicebus:Queue
///     name: example
///     properties:
///       name: exampleQueue
///       namespaceId: ${exampleNamespace.id}
///       enablePartitioning: true
///   exampleQueueAuthorizationRule:
///     type: azure:servicebus:QueueAuthorizationRule
///     name: example
///     properties:
///       name: exampleRule
///       queueId: ${exampleQueue.id}
///       listen: false
///       send: true
///       manage: false
///   exampleIoTHub:
///     type: azure:iot:IoTHub
///     name: example
///     properties:
///       name: exampleIothub
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku:
///         name: B1
///         capacity: '1'
///       tags:
///         purpose: example
///   exampleEndpointServicebusQueue:
///     type: azure:iot:EndpointServicebusQueue
///     name: example
///     properties:
///       resourceGroupName: ${example.name}
///       iothubId: ${exampleIoTHub.id}
///       name: example
///       connectionString: ${exampleQueueAuthorizationRule.primaryConnectionString}
/// ```
///
/// ## Import
///
/// IoTHub ServiceBus Queue Endpoint can be imported using the `resource id`, e.g.
///
/// g
///
/// ```sh
/// $ pulumi import azure:iot/endpointServicebusQueue:EndpointServicebusQueue servicebus_queue1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Devices/iotHubs/hub1/endpoints/servicebusqueue_endpoint1
/// ```
///
pub mod endpoint_servicebus_queue {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EndpointServicebusQueueArgs {
        /// Type used to authenticate against the Service Bus Queue endpoint. Possible values are `keyBased` and `identityBased`. Defaults to `keyBased`.
        #[builder(into, default)]
        pub authentication_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The connection string for the endpoint. This attribute can only be specified and is mandatory when `authentication_type` is `keyBased`.
        #[builder(into, default)]
        pub connection_string: pulumi_wasm_rust::Output<Option<String>>,
        /// URI of the Service Bus endpoint. This attribute can only be specified and is mandatory when `authentication_type` is `identityBased`.
        #[builder(into, default)]
        pub endpoint_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the Service Bus Queue. This attribute can only be specified and is mandatory when `authentication_type` is `identityBased`.
        #[builder(into, default)]
        pub entity_path: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the User Managed Identity used to authenticate against the Service Bus Queue endpoint.
        ///
        /// > **NOTE:** `identity_id` can only be specified when `authentication_type` is `identityBased`. It must be one of the `identity_ids` of the Iot Hub. If not specified when `authentication_type` is `identityBased`, System Assigned Managed Identity of the Iot Hub will be used.
        #[builder(into, default)]
        pub identity_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The IoTHub ID for the endpoint. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iothub_id: pulumi_wasm_rust::Output<String>,
        /// The name of the endpoint. The name must be unique across endpoint types. The following names are reserved: `events`, `operationsMonitoringEvents`, `fileNotifications` and `$default`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group under which the Service Bus Queue has been created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct EndpointServicebusQueueResult {
        /// Type used to authenticate against the Service Bus Queue endpoint. Possible values are `keyBased` and `identityBased`. Defaults to `keyBased`.
        pub authentication_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The connection string for the endpoint. This attribute can only be specified and is mandatory when `authentication_type` is `keyBased`.
        pub connection_string: pulumi_wasm_rust::Output<Option<String>>,
        /// URI of the Service Bus endpoint. This attribute can only be specified and is mandatory when `authentication_type` is `identityBased`.
        pub endpoint_uri: pulumi_wasm_rust::Output<Option<String>>,
        /// Name of the Service Bus Queue. This attribute can only be specified and is mandatory when `authentication_type` is `identityBased`.
        pub entity_path: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the User Managed Identity used to authenticate against the Service Bus Queue endpoint.
        ///
        /// > **NOTE:** `identity_id` can only be specified when `authentication_type` is `identityBased`. It must be one of the `identity_ids` of the Iot Hub. If not specified when `authentication_type` is `identityBased`, System Assigned Managed Identity of the Iot Hub will be used.
        pub identity_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The IoTHub ID for the endpoint. Changing this forces a new resource to be created.
        pub iothub_id: pulumi_wasm_rust::Output<String>,
        /// The name of the endpoint. The name must be unique across endpoint types. The following names are reserved: `events`, `operationsMonitoringEvents`, `fileNotifications` and `$default`. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group under which the Service Bus Queue has been created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: EndpointServicebusQueueArgs,
    ) -> EndpointServicebusQueueResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let authentication_type_binding = args.authentication_type.get_inner();
        let connection_string_binding = args.connection_string.get_inner();
        let endpoint_uri_binding = args.endpoint_uri.get_inner();
        let entity_path_binding = args.entity_path.get_inner();
        let identity_id_binding = args.identity_id.get_inner();
        let iothub_id_binding = args.iothub_id.get_inner();
        let name_binding = args.name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:iot/endpointServicebusQueue:EndpointServicebusQueue".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "authenticationType".into(),
                    value: &authentication_type_binding,
                },
                register_interface::ObjectField {
                    name: "connectionString".into(),
                    value: &connection_string_binding,
                },
                register_interface::ObjectField {
                    name: "endpointUri".into(),
                    value: &endpoint_uri_binding,
                },
                register_interface::ObjectField {
                    name: "entityPath".into(),
                    value: &entity_path_binding,
                },
                register_interface::ObjectField {
                    name: "identityId".into(),
                    value: &identity_id_binding,
                },
                register_interface::ObjectField {
                    name: "iothubId".into(),
                    value: &iothub_id_binding,
                },
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
                    name: "authenticationType".into(),
                },
                register_interface::ResultField {
                    name: "connectionString".into(),
                },
                register_interface::ResultField {
                    name: "endpointUri".into(),
                },
                register_interface::ResultField {
                    name: "entityPath".into(),
                },
                register_interface::ResultField {
                    name: "identityId".into(),
                },
                register_interface::ResultField {
                    name: "iothubId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        EndpointServicebusQueueResult {
            authentication_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("authenticationType").unwrap(),
            ),
            connection_string: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionString").unwrap(),
            ),
            endpoint_uri: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointUri").unwrap(),
            ),
            entity_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("entityPath").unwrap(),
            ),
            identity_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identityId").unwrap(),
            ),
            iothub_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iothubId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
        }
    }
}
