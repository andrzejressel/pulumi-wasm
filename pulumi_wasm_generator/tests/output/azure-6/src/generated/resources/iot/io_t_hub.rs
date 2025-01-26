/// Manages an IotHub
///
/// > **NOTE:** Endpoints can be defined either directly on the `azure.iot.IoTHub` resource, or using the `azurerm_iothub_endpoint_*` resources - but the two ways of defining the endpoints cannot be used together. If both are used against the same IoTHub, spurious changes will occur. Also, defining a `azurerm_iothub_endpoint_*` resource and another endpoint of a different type directly on the `azure.iot.IoTHub` resource is not supported.
///
/// > **NOTE:** Routes can be defined either directly on the `azure.iot.IoTHub` resource, or using the `azure.iot.Route` resource - but the two cannot be used together. If both are used against the same IoTHub, spurious changes will occur.
///
/// > **NOTE:** Enrichments can be defined either directly on the `azure.iot.IoTHub` resource, or using the `azure.iot.Enrichment` resource - but the two cannot be used together. If both are used against the same IoTHub, spurious changes will occur.
///
/// > **NOTE:** Fallback route can be defined either directly on the `azure.iot.IoTHub` resource, or using the `azure.iot.FallbackRoute` resource - but the two cannot be used together. If both are used against the same IoTHub, spurious changes will occur.
///
/// > **NOTE:** File upload can be defined either directly on the `azure.iot.IoTHub` resource, or using the `azure.iot.FileUpload` resource - but the two cannot be used together. If both are used against the same IoTHub, spurious changes will occur.
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
///       name: examplestorage
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       accountTier: Standard
///       accountReplicationType: LRS
///   exampleContainer:
///     type: azure:storage:Container
///     name: example
///     properties:
///       name: examplecontainer
///       storageAccountName: ${exampleAccount.name}
///       containerAccessType: private
///   exampleEventHubNamespace:
///     type: azure:eventhub:EventHubNamespace
///     name: example
///     properties:
///       name: example-namespace
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku: Basic
///   exampleEventHub:
///     type: azure:eventhub:EventHub
///     name: example
///     properties:
///       name: example-eventhub
///       resourceGroupName: ${example.name}
///       namespaceName: ${exampleEventHubNamespace.name}
///       partitionCount: 2
///       messageRetention: 1
///   exampleAuthorizationRule:
///     type: azure:eventhub:AuthorizationRule
///     name: example
///     properties:
///       resourceGroupName: ${example.name}
///       namespaceName: ${exampleEventHubNamespace.name}
///       eventhubName: ${exampleEventHub.name}
///       name: acctest
///       send: true
///   exampleIoTHub:
///     type: azure:iot:IoTHub
///     name: example
///     properties:
///       name: Example-IoTHub
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       localAuthenticationEnabled: false
///       sku:
///         name: S1
///         capacity: '1'
///       endpoints:
///         - type: AzureIotHub.StorageContainer
///           connectionString: ${exampleAccount.primaryBlobConnectionString}
///           name: export
///           batchFrequencyInSeconds: 60
///           maxChunkSizeInBytes: 1.048576e+07
///           containerName: ${exampleContainer.name}
///           encoding: Avro
///           fileNameFormat: '{iothub}/{partition}_{YYYY}_{MM}_{DD}_{HH}_{mm}'
///         - type: AzureIotHub.EventHub
///           connectionString: ${exampleAuthorizationRule.primaryConnectionString}
///           name: export2
///       routes:
///         - name: export
///           source: DeviceMessages
///           condition: 'true'
///           endpointNames:
///             - export
///           enabled: true
///         - name: export2
///           source: DeviceMessages
///           condition: 'true'
///           endpointNames:
///             - export2
///           enabled: true
///       enrichments:
///         - key: tenant
///           value: $twin.tags.Tenant
///           endpointNames:
///             - export
///             - export2
///       cloudToDevice:
///         maxDeliveryCount: 30
///         defaultTtl: PT1H
///         feedbacks:
///           - timeToLive: PT1H10M
///             maxDeliveryCount: 15
///             lockDuration: PT30S
///       tags:
///         purpose: testing
/// ```
///
/// ## Import
///
/// IoTHubs can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:iot/ioTHub:IoTHub hub1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Devices/iotHubs/hub1
/// ```
///
pub mod io_t_hub {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IoTHubArgs {
        #[builder(into, default)]
        pub cloud_to_device: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::iot::IoTHubCloudToDevice>,
        >,
        /// An `endpoint` block as defined below.
        #[builder(into, default)]
        pub endpoints: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::IoTHubEndpoint>>,
        >,
        #[builder(into, default)]
        pub enrichments: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::IoTHubEnrichment>>,
        >,
        /// The number of device-to-cloud partitions used by backing event hubs. Must be between `2` and `128`. Defaults to `4`.
        #[builder(into, default)]
        pub event_hub_partition_count: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The event hub retention to use in days. Must be between `1` and `7`. Defaults to `1`.
        #[builder(into, default)]
        pub event_hub_retention_in_days: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// A `fallback_route` block as defined below. If the fallback route is enabled, messages that don't match any of the supplied routes are automatically sent to this route. Defaults to messages/events.
        ///
        /// > **NOTE:** If `fallback_route` isn't explicitly specified, the fallback route wouldn't be enabled by default.
        #[builder(into, default)]
        pub fallback_route: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::iot::IoTHubFallbackRoute>,
        >,
        /// A `file_upload` block as defined below.
        #[builder(into, default)]
        pub file_upload: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::iot::IoTHubFileUpload>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::iot::IoTHubIdentity>,
        >,
        /// If false, SAS tokens with Iot hub scoped SAS keys cannot be used for authentication. Defaults to `true`.
        #[builder(into, default)]
        pub local_authentication_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies the supported Azure location where the resource has to be created. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub min_tls_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the IotHub resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `network_rule_set` block as defined below.
        #[builder(into, default)]
        pub network_rule_sets: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::IoTHubNetworkRuleSet>>,
        >,
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name of the resource group under which the IotHub resource has to be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub routes: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::iot::IoTHubRoute>>,
        >,
        /// A `sku` block as defined below.
        #[builder(into)]
        pub sku: pulumi_wasm_rust::InputOrOutput<super::super::types::iot::IoTHubSku>,
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct IoTHubResult {
        pub cloud_to_device: pulumi_wasm_rust::Output<
            super::super::types::iot::IoTHubCloudToDevice,
        >,
        /// An `endpoint` block as defined below.
        pub endpoints: pulumi_wasm_rust::Output<
            Vec<super::super::types::iot::IoTHubEndpoint>,
        >,
        pub enrichments: pulumi_wasm_rust::Output<
            Vec<super::super::types::iot::IoTHubEnrichment>,
        >,
        /// The EventHub compatible endpoint for events data
        pub event_hub_events_endpoint: pulumi_wasm_rust::Output<String>,
        /// The EventHub namespace for events data
        pub event_hub_events_namespace: pulumi_wasm_rust::Output<String>,
        /// The EventHub compatible path for events data
        pub event_hub_events_path: pulumi_wasm_rust::Output<String>,
        /// The EventHub compatible endpoint for operational data
        pub event_hub_operations_endpoint: pulumi_wasm_rust::Output<String>,
        /// The EventHub compatible path for operational data
        pub event_hub_operations_path: pulumi_wasm_rust::Output<String>,
        /// The number of device-to-cloud partitions used by backing event hubs. Must be between `2` and `128`. Defaults to `4`.
        pub event_hub_partition_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The event hub retention to use in days. Must be between `1` and `7`. Defaults to `1`.
        pub event_hub_retention_in_days: pulumi_wasm_rust::Output<Option<i32>>,
        /// A `fallback_route` block as defined below. If the fallback route is enabled, messages that don't match any of the supplied routes are automatically sent to this route. Defaults to messages/events.
        ///
        /// > **NOTE:** If `fallback_route` isn't explicitly specified, the fallback route wouldn't be enabled by default.
        pub fallback_route: pulumi_wasm_rust::Output<
            super::super::types::iot::IoTHubFallbackRoute,
        >,
        /// A `file_upload` block as defined below.
        pub file_upload: pulumi_wasm_rust::Output<
            Option<super::super::types::iot::IoTHubFileUpload>,
        >,
        /// The hostname of the IotHub Resource.
        pub hostname: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::iot::IoTHubIdentity>,
        >,
        /// If false, SAS tokens with Iot hub scoped SAS keys cannot be used for authentication. Defaults to `true`.
        pub local_authentication_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource has to be created. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        pub min_tls_version: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the IotHub resource. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `network_rule_set` block as defined below.
        pub network_rule_sets: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::iot::IoTHubNetworkRuleSet>>,
        >,
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group under which the IotHub resource has to be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        pub routes: pulumi_wasm_rust::Output<Vec<super::super::types::iot::IoTHubRoute>>,
        /// One or more `shared_access_policy` blocks as defined below.
        pub shared_access_policies: pulumi_wasm_rust::Output<
            Vec<super::super::types::iot::IoTHubSharedAccessPolicy>,
        >,
        /// A `sku` block as defined below.
        pub sku: pulumi_wasm_rust::Output<super::super::types::iot::IoTHubSku>,
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: IoTHubArgs,
    ) -> IoTHubResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cloud_to_device_binding = args
            .cloud_to_device
            .get_output(context)
            .get_inner();
        let endpoints_binding = args.endpoints.get_output(context).get_inner();
        let enrichments_binding = args.enrichments.get_output(context).get_inner();
        let event_hub_partition_count_binding = args
            .event_hub_partition_count
            .get_output(context)
            .get_inner();
        let event_hub_retention_in_days_binding = args
            .event_hub_retention_in_days
            .get_output(context)
            .get_inner();
        let fallback_route_binding = args.fallback_route.get_output(context).get_inner();
        let file_upload_binding = args.file_upload.get_output(context).get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let local_authentication_enabled_binding = args
            .local_authentication_enabled
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let min_tls_version_binding = args
            .min_tls_version
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_rule_sets_binding = args
            .network_rule_sets
            .get_output(context)
            .get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let routes_binding = args.routes.get_output(context).get_inner();
        let sku_binding = args.sku.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:iot/ioTHub:IoTHub".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cloudToDevice".into(),
                    value: &cloud_to_device_binding,
                },
                register_interface::ObjectField {
                    name: "endpoints".into(),
                    value: &endpoints_binding,
                },
                register_interface::ObjectField {
                    name: "enrichments".into(),
                    value: &enrichments_binding,
                },
                register_interface::ObjectField {
                    name: "eventHubPartitionCount".into(),
                    value: &event_hub_partition_count_binding,
                },
                register_interface::ObjectField {
                    name: "eventHubRetentionInDays".into(),
                    value: &event_hub_retention_in_days_binding,
                },
                register_interface::ObjectField {
                    name: "fallbackRoute".into(),
                    value: &fallback_route_binding,
                },
                register_interface::ObjectField {
                    name: "fileUpload".into(),
                    value: &file_upload_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "localAuthenticationEnabled".into(),
                    value: &local_authentication_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "minTlsVersion".into(),
                    value: &min_tls_version_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "networkRuleSets".into(),
                    value: &network_rule_sets_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "routes".into(),
                    value: &routes_binding,
                },
                register_interface::ObjectField {
                    name: "sku".into(),
                    value: &sku_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "cloudToDevice".into(),
                },
                register_interface::ResultField {
                    name: "endpoints".into(),
                },
                register_interface::ResultField {
                    name: "enrichments".into(),
                },
                register_interface::ResultField {
                    name: "eventHubEventsEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "eventHubEventsNamespace".into(),
                },
                register_interface::ResultField {
                    name: "eventHubEventsPath".into(),
                },
                register_interface::ResultField {
                    name: "eventHubOperationsEndpoint".into(),
                },
                register_interface::ResultField {
                    name: "eventHubOperationsPath".into(),
                },
                register_interface::ResultField {
                    name: "eventHubPartitionCount".into(),
                },
                register_interface::ResultField {
                    name: "eventHubRetentionInDays".into(),
                },
                register_interface::ResultField {
                    name: "fallbackRoute".into(),
                },
                register_interface::ResultField {
                    name: "fileUpload".into(),
                },
                register_interface::ResultField {
                    name: "hostname".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "localAuthenticationEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "minTlsVersion".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "networkRuleSets".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "routes".into(),
                },
                register_interface::ResultField {
                    name: "sharedAccessPolicies".into(),
                },
                register_interface::ResultField {
                    name: "sku".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "type".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        IoTHubResult {
            cloud_to_device: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudToDevice").unwrap(),
            ),
            endpoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoints").unwrap(),
            ),
            enrichments: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enrichments").unwrap(),
            ),
            event_hub_events_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventHubEventsEndpoint").unwrap(),
            ),
            event_hub_events_namespace: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventHubEventsNamespace").unwrap(),
            ),
            event_hub_events_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventHubEventsPath").unwrap(),
            ),
            event_hub_operations_endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventHubOperationsEndpoint").unwrap(),
            ),
            event_hub_operations_path: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventHubOperationsPath").unwrap(),
            ),
            event_hub_partition_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventHubPartitionCount").unwrap(),
            ),
            event_hub_retention_in_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("eventHubRetentionInDays").unwrap(),
            ),
            fallback_route: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fallbackRoute").unwrap(),
            ),
            file_upload: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fileUpload").unwrap(),
            ),
            hostname: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hostname").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            local_authentication_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localAuthenticationEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            min_tls_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("minTlsVersion").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network_rule_sets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkRuleSets").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            routes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routes").unwrap(),
            ),
            shared_access_policies: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sharedAccessPolicies").unwrap(),
            ),
            sku: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sku").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("type").unwrap(),
            ),
        }
    }
}
