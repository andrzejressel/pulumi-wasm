/// Manages an IotHub Enrichment
///
/// > **NOTE:** Enrichment can be defined either directly on the `azure.iot.IoTHub` resource, or using the `azure.iot.Enrichment` resources - but the two cannot be used together. If both are used against the same IoTHub, spurious changes will occur.
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
///   exampleRoute:
///     type: azure:iot:Route
///     name: example
///     properties:
///       resourceGroupName: ${example.name}
///       iothubName: ${exampleIoTHub.name}
///       name: example
///       source: DeviceMessages
///       condition: 'true'
///       endpointNames: ${exampleEndpointStorageContainer.name}
///       enabled: true
///   exampleEnrichment:
///     type: azure:iot:Enrichment
///     name: example
///     properties:
///       resourceGroupName: ${example.name}
///       iothubName: ${exampleIoTHub.name}
///       key: example
///       value: my value
///       endpointNames:
///         - ${exampleEndpointStorageContainer.name}
/// ```
///
/// ## Import
///
/// IoTHub Enrichment can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:iot/enrichment:Enrichment enrichment1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Devices/iotHubs/hub1/enrichments/enrichment1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod enrichment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EnrichmentArgs {
        /// The list of endpoints which will be enriched.
        #[builder(into)]
        pub endpoint_names: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The IoTHub name of the enrichment. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iothub_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The key of the enrichment. Changing this forces a new resource to be created.
        #[builder(into)]
        pub key: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group under which the IoTHub resource is created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The value of the enrichment. Value can be any static string, the name of the IoT hub sending the message (use `$iothubname`) or information from the device twin (ex: `$twin.tags.latitude`)
        #[builder(into)]
        pub value: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EnrichmentResult {
        /// The list of endpoints which will be enriched.
        pub endpoint_names: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The IoTHub name of the enrichment. Changing this forces a new resource to be created.
        pub iothub_name: pulumi_gestalt_rust::Output<String>,
        /// The key of the enrichment. Changing this forces a new resource to be created.
        pub key: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group under which the IoTHub resource is created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The value of the enrichment. Value can be any static string, the name of the IoT hub sending the message (use `$iothubname`) or information from the device twin (ex: `$twin.tags.latitude`)
        pub value: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EnrichmentArgs,
    ) -> EnrichmentResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let endpoint_names_binding_1 = args.endpoint_names.get_output(context);
        let endpoint_names_binding = endpoint_names_binding_1.get_inner();
        let iothub_name_binding_1 = args.iothub_name.get_output(context);
        let iothub_name_binding = iothub_name_binding_1.get_inner();
        let key_binding_1 = args.key.get_output(context);
        let key_binding = key_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let value_binding_1 = args.value.get_output(context);
        let value_binding = value_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:iot/enrichment:Enrichment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "endpointNames".into(),
                    value: &endpoint_names_binding,
                },
                register_interface::ObjectField {
                    name: "iothubName".into(),
                    value: &iothub_name_binding,
                },
                register_interface::ObjectField {
                    name: "key".into(),
                    value: &key_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "value".into(),
                    value: &value_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EnrichmentResult {
            endpoint_names: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpointNames"),
            ),
            iothub_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iothubName"),
            ),
            key: pulumi_gestalt_rust::__private::into_domain(o.extract_field("key")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            value: pulumi_gestalt_rust::__private::into_domain(o.extract_field("value")),
        }
    }
}
