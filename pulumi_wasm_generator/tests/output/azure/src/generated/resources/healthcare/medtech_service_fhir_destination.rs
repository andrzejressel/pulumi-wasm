/// Manages a Healthcare Med Tech Service Fhir Destination.
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: West Europe
///   exampleWorkspace:
///     type: azure:healthcare:Workspace
///     name: example
///     properties:
///       name: exampleworkspace
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleEventHubNamespace:
///     type: azure:eventhub:EventHubNamespace
///     name: example
///     properties:
///       name: example-ehn
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku: Standard
///   exampleEventHub:
///     type: azure:eventhub:EventHub
///     name: example
///     properties:
///       name: example-eh
///       namespaceName: ${exampleEventHubNamespace.name}
///       resourceGroupName: ${example.name}
///       partitionCount: 1
///       messageRetention: 1
///   exampleConsumerGroup:
///     type: azure:eventhub:ConsumerGroup
///     name: example
///     properties:
///       name: $default
///       namespaceName: ${exampleEventHubNamespace.name}
///       eventhubName: ${exampleEventHub.name}
///       resourceGroupName: ${example.name}
///   exampleFhirService:
///     type: azure:healthcare:FhirService
///     name: example
///     properties:
///       name: examplefhir
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       workspaceId: ${exampleWorkspace.id}
///       kind: fhir-R4
///       authentication:
///         authority: https://login.microsoftonline.com/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
///         audience: https://examplefhir.fhir.azurehealthcareapis.com
///   exampleMedtechService:
///     type: azure:healthcare:MedtechService
///     name: example
///     properties:
///       name: examplemt
///       workspaceId: ${exampleWorkspace.id}
///       location: ${example.location}
///       eventhubNamespaceName: ${exampleEventHubNamespace.name}
///       eventhubName: ${exampleEventHub.name}
///       eventhubConsumerGroupName: ${exampleConsumerGroup.name}
///       deviceMappingJson:
///         fn::toJSON:
///           templateType: CollectionContent
///           template: []
///   exampleMedtechServiceFhirDestination:
///     type: azure:healthcare:MedtechServiceFhirDestination
///     name: example
///     properties:
///       name: examplemtdes
///       location: east us
///       medtechServiceId: ${exampleMedtechService.id}
///       destinationFhirServiceId: ${exampleFhirService.id}
///       destinationIdentityResolutionType: Create
///       destinationFhirMappingJson:
///         fn::toJSON:
///           templateType: CollectionFhirTemplate
///           template:
///             - templateType: CodeValueFhir
///               template:
///                 codes:
///                   - code: 8867-4
///                     system: http://loinc.org
///                     display: Heart rate
///                 periodInterval: 60
///                 typeName: heartrate
///                 value:
///                   defaultPeriod: 5000
///                   unit: count/min
///                   valueName: hr
///                   valueType: SampledData
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Healthcare Med Tech Service Fhir Destination can be imported using the resource`id`, e.g.
///
/// ```sh
/// $ pulumi import azure:healthcare/medtechServiceFhirDestination:MedtechServiceFhirDestination example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.HealthcareApis/workspaces/workspace1/iotConnectors/iotconnector1/fhirDestinations/destination1
/// ```
///
pub mod medtech_service_fhir_destination {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MedtechServiceFhirDestinationArgs {
        /// Specifies the destination Fhir mappings of the Med Tech Service Fhir Destination.
        #[builder(into)]
        pub destination_fhir_mapping_json: pulumi_wasm_rust::Output<String>,
        /// Specifies the destination fhir service id of the Med Tech Service Fhir Destination.
        #[builder(into)]
        pub destination_fhir_service_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the destination identity resolution type where the Healthcare Med Tech Service Fhir Destination should be created. Possible values are `Create`, `Lookup`.
        #[builder(into)]
        pub destination_identity_resolution_type: pulumi_wasm_rust::Output<String>,
        /// Specifies the Azure Region where the Healthcare Med Tech Service Fhir Destination should be created. Changing this forces a new Healthcare Med Tech Service Fhir Destination to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name of the Healthcare Med Tech Service where the Healthcare Med Tech Service Fhir Destination should exist. Changing this forces a new Healthcare Med Tech Service Fhir Destination to be created.
        #[builder(into)]
        pub medtech_service_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Healthcare Med Tech Service Fhir Destination. Changing this forces a new Healthcare Med Tech Service Fhir Destination to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MedtechServiceFhirDestinationResult {
        /// Specifies the destination Fhir mappings of the Med Tech Service Fhir Destination.
        pub destination_fhir_mapping_json: pulumi_wasm_rust::Output<String>,
        /// Specifies the destination fhir service id of the Med Tech Service Fhir Destination.
        pub destination_fhir_service_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the destination identity resolution type where the Healthcare Med Tech Service Fhir Destination should be created. Possible values are `Create`, `Lookup`.
        pub destination_identity_resolution_type: pulumi_wasm_rust::Output<String>,
        /// Specifies the Azure Region where the Healthcare Med Tech Service Fhir Destination should be created. Changing this forces a new Healthcare Med Tech Service Fhir Destination to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Healthcare Med Tech Service where the Healthcare Med Tech Service Fhir Destination should exist. Changing this forces a new Healthcare Med Tech Service Fhir Destination to be created.
        pub medtech_service_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Healthcare Med Tech Service Fhir Destination. Changing this forces a new Healthcare Med Tech Service Fhir Destination to be created.
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: MedtechServiceFhirDestinationArgs,
    ) -> MedtechServiceFhirDestinationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let destination_fhir_mapping_json_binding = args
            .destination_fhir_mapping_json
            .get_inner();
        let destination_fhir_service_id_binding = args
            .destination_fhir_service_id
            .get_inner();
        let destination_identity_resolution_type_binding = args
            .destination_identity_resolution_type
            .get_inner();
        let location_binding = args.location.get_inner();
        let medtech_service_id_binding = args.medtech_service_id.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:healthcare/medtechServiceFhirDestination:MedtechServiceFhirDestination"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "destinationFhirMappingJson".into(),
                    value: &destination_fhir_mapping_json_binding,
                },
                register_interface::ObjectField {
                    name: "destinationFhirServiceId".into(),
                    value: &destination_fhir_service_id_binding,
                },
                register_interface::ObjectField {
                    name: "destinationIdentityResolutionType".into(),
                    value: &destination_identity_resolution_type_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "medtechServiceId".into(),
                    value: &medtech_service_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "destinationFhirMappingJson".into(),
                },
                register_interface::ResultField {
                    name: "destinationFhirServiceId".into(),
                },
                register_interface::ResultField {
                    name: "destinationIdentityResolutionType".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "medtechServiceId".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MedtechServiceFhirDestinationResult {
            destination_fhir_mapping_json: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationFhirMappingJson").unwrap(),
            ),
            destination_fhir_service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationFhirServiceId").unwrap(),
            ),
            destination_identity_resolution_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationIdentityResolutionType").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            medtech_service_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("medtechServiceId").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}