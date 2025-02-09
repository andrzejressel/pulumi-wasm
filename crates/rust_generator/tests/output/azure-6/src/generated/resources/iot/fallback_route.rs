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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod fallback_route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FallbackRouteArgs {
        /// The condition that is evaluated to apply the routing rule. For grammar, see: <https://docs.microsoft.com/azure/iot-hub/iot-hub-devguide-query-language>. Defaults to `true`.
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Used to specify whether the fallback route is enabled.
        #[builder(into)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The endpoints to which messages that satisfy the condition are routed. Currently only 1 endpoint is allowed.
        #[builder(into)]
        pub endpoint_names: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the IoTHub to which this Fallback Route belongs. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iothub_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group under which the IotHub Storage Container Endpoint resource has to be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The source that the routing rule is to be applied to. Possible values include: `DeviceConnectionStateEvents`, `DeviceJobLifecycleEvents`, `DeviceLifecycleEvents`, `DeviceMessages`, `DigitalTwinChangeEvents`, `Invalid`, `TwinChangeEvents`. Defaults to `DeviceMessages`.
        #[builder(into, default)]
        pub source: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct FallbackRouteResult {
        /// The condition that is evaluated to apply the routing rule. For grammar, see: <https://docs.microsoft.com/azure/iot-hub/iot-hub-devguide-query-language>. Defaults to `true`.
        pub condition: pulumi_gestalt_rust::Output<Option<String>>,
        /// Used to specify whether the fallback route is enabled.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The endpoints to which messages that satisfy the condition are routed. Currently only 1 endpoint is allowed.
        pub endpoint_names: pulumi_gestalt_rust::Output<String>,
        /// The name of the IoTHub to which this Fallback Route belongs. Changing this forces a new resource to be created.
        pub iothub_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group under which the IotHub Storage Container Endpoint resource has to be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The source that the routing rule is to be applied to. Possible values include: `DeviceConnectionStateEvents`, `DeviceJobLifecycleEvents`, `DeviceLifecycleEvents`, `DeviceMessages`, `DigitalTwinChangeEvents`, `Invalid`, `TwinChangeEvents`. Defaults to `DeviceMessages`.
        pub source: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FallbackRouteArgs,
    ) -> FallbackRouteResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let condition_binding_1 = args.condition.get_output(context);
        let condition_binding = condition_binding_1.get_inner();
        let enabled_binding_1 = args.enabled.get_output(context);
        let enabled_binding = enabled_binding_1.get_inner();
        let endpoint_names_binding_1 = args.endpoint_names.get_output(context);
        let endpoint_names_binding = endpoint_names_binding_1.get_inner();
        let iothub_name_binding_1 = args.iothub_name.get_output(context);
        let iothub_name_binding = iothub_name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let source_binding_1 = args.source.get_output(context);
        let source_binding = source_binding_1.get_inner();
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        FallbackRouteResult {
            condition: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("condition"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            endpoint_names: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointNames"),
            ),
            iothub_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iothubName"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            source: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("source"),
            ),
        }
    }
}
