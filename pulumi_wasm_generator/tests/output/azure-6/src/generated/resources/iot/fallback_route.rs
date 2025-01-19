/// Manages an IotHub Fallback Route
///
/// ## Disclaimers
///
/// > **Note:** Fallback route can be defined either directly on the `azure.iot.IoTHub` resource, or using the `azure.iot.FallbackRoute` resource - but the two cannot be used together. If both are used against the same IoTHub, spurious changes will occur.
///
/// > **Note:** Since this resource is provisioned by default, the Azure Provider will not check for the presence of an existing resource prior to attempting to create it.
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
///   exampleAccount:
///     type: azure:storage:Account
///     name: example
///     properties:
///       name: examplestorageaccount
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleContainer:
///     type: azure:storage:Container
///     name: example
///     properties:
///       name: example
///       storageAccountName: ${exampleAccount.name}
///       containerAccessType: private
///   exampleIoTHub:
///     type: azure:iot:IoTHub
///     name: example
///     properties:
///       name: exampleIothub
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku:
///         name: S1
///         capacity: '1'
///       tags:
///         purpose: testing
///   exampleEndpointStorageContainer:
///     type: azure:iot:EndpointStorageContainer
///     name: example
///     properties:
///       resourceGroupName: ${example.name}
///       iothubId: ${exampleIoTHub.id}
///       name: example
///       connectionString: ${exampleAccount.primaryBlobConnectionString}
///       batchFrequencyInSeconds: 60
///       maxChunkSizeInBytes: 1.048576e+07
///       containerName: ${exampleContainer.name}
///       encoding: Avro
///       fileNameFormat: '{iothub}/{partition}_{YYYY}_{MM}_{DD}_{HH}_{mm}'
///   exampleFallbackRoute:
///     type: azure:iot:FallbackRoute
///     name: example
///     properties:
///       resourceGroupName: ${example.name}
///       iothubName: ${exampleIoTHub.name}
///       condition: 'true'
///       endpointNames: ${exampleEndpointStorageContainer.name}
///       enabled: true
/// ```
///
/// ## Import
///
/// IoTHub Fallback Route can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:iot/fallbackRoute:FallbackRoute route1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Devices/iotHubs/hub1/fallbackRoute/default
/// ```
///
pub mod fallback_route {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FallbackRouteArgs {
        /// The condition that is evaluated to apply the routing rule. For grammar, see: <https://docs.microsoft.com/azure/iot-hub/iot-hub-devguide-query-language>. Defaults to `true`.
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::Output<Option<String>>,
        /// Used to specify whether the fallback route is enabled.
        #[builder(into)]
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The endpoints to which messages that satisfy the condition are routed. Currently only 1 endpoint is allowed.
        #[builder(into)]
        pub endpoint_names: pulumi_wasm_rust::Output<String>,
        /// The name of the IoTHub to which this Fallback Route belongs. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iothub_name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group under which the IotHub Storage Container Endpoint resource has to be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The source that the routing rule is to be applied to. Possible values include: `DeviceConnectionStateEvents`, `DeviceJobLifecycleEvents`, `DeviceLifecycleEvents`, `DeviceMessages`, `DigitalTwinChangeEvents`, `Invalid`, `TwinChangeEvents`. Defaults to `DeviceMessages`.
        #[builder(into, default)]
        pub source: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FallbackRouteResult {
        /// The condition that is evaluated to apply the routing rule. For grammar, see: <https://docs.microsoft.com/azure/iot-hub/iot-hub-devguide-query-language>. Defaults to `true`.
        pub condition: pulumi_wasm_rust::Output<Option<String>>,
        /// Used to specify whether the fallback route is enabled.
        pub enabled: pulumi_wasm_rust::Output<bool>,
        /// The endpoints to which messages that satisfy the condition are routed. Currently only 1 endpoint is allowed.
        pub endpoint_names: pulumi_wasm_rust::Output<String>,
        /// The name of the IoTHub to which this Fallback Route belongs. Changing this forces a new resource to be created.
        pub iothub_name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group under which the IotHub Storage Container Endpoint resource has to be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The source that the routing rule is to be applied to. Possible values include: `DeviceConnectionStateEvents`, `DeviceJobLifecycleEvents`, `DeviceLifecycleEvents`, `DeviceMessages`, `DigitalTwinChangeEvents`, `Invalid`, `TwinChangeEvents`. Defaults to `DeviceMessages`.
        pub source: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: FallbackRouteArgs) -> FallbackRouteResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let endpoint_names_binding = args.endpoint_names.get_inner();
        let iothub_name_binding = args.iothub_name.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let source_binding = args.source.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:iot/fallbackRoute:FallbackRoute".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "endpointNames".into(),
                    value: &endpoint_names_binding,
                },
                register_interface::ObjectField {
                    name: "iothubName".into(),
                    value: &iothub_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "source".into(),
                    value: &source_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "condition".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "endpointNames".into(),
                },
                register_interface::ResultField {
                    name: "iothubName".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "source".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        FallbackRouteResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("condition").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            endpoint_names: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointNames").unwrap(),
            ),
            iothub_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("iothubName").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            source: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("source").unwrap(),
            ),
        }
    }
}
