/// Manages a Network Function Collector Policy.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West US 2
///   exampleExpressRoutePort:
///     type: azure:network:ExpressRoutePort
///     name: example
///     properties:
///       name: example-erp
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       peeringLocation: Equinix-Seattle-SE2
///       bandwidthInGbps: 10
///       encapsulation: Dot1Q
///   exampleExpressRouteCircuit:
///     type: azure:network:ExpressRouteCircuit
///     name: example
///     properties:
///       name: example-erc
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       expressRoutePortId: ${exampleExpressRoutePort.id}
///       bandwidthInGbps: 1
///       sku:
///         tier: Standard
///         family: MeteredData
///   exampleExpressRouteCircuitPeering:
///     type: azure:network:ExpressRouteCircuitPeering
///     name: example
///     properties:
///       peeringType: MicrosoftPeering
///       expressRouteCircuitName: ${exampleExpressRouteCircuit.name}
///       resourceGroupName: ${example.name}
///       peerAsn: 100
///       primaryPeerAddressPrefix: 192.168.199.0/30
///       secondaryPeerAddressPrefix: 192.168.200.0/30
///       vlanId: 300
///       microsoftPeeringConfig:
///         advertisedPublicPrefixes:
///           - 123.6.0.0/24
///   exampleAzureTrafficCollector:
///     type: azure:networkfunction:AzureTrafficCollector
///     name: example
///     properties:
///       name: example-nfatc
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///     options:
///       dependsOn:
///         - ${exampleExpressRouteCircuitPeering}
///   exampleCollectorPolicy:
///     type: azure:networkfunction:CollectorPolicy
///     name: example
///     properties:
///       name: example-nfcp
///       trafficCollectorId: ${exampleAzureTrafficCollector.id}
///       location: ${example.location}
///       ipfxEmission:
///         destinationTypes: AzureMonitor
///       ipfxIngestion:
///         sourceResourceIds:
///           - ${exampleExpressRouteCircuit.id}
///       tags:
///         key: value
/// ```
///
/// ## Import
///
/// Network Function Collector Policy can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:networkfunction/collectorPolicy:CollectorPolicy example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.NetworkFunction/azureTrafficCollectors/azureTrafficCollector1/collectorPolicies/collectorPolicy1
/// ```
///
pub mod collector_policy {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CollectorPolicyArgs {
        /// An `ipfx_emission` block as defined below. Changing this forces a new Network Function Collector Policy to be created.
        #[builder(into)]
        pub ipfx_emission: pulumi_wasm_rust::Output<
            super::super::types::networkfunction::CollectorPolicyIpfxEmission,
        >,
        /// An `ipfx_ingestion` block as defined below. Changing this forces a new Network Function Collector Policy to be created.
        #[builder(into)]
        pub ipfx_ingestion: pulumi_wasm_rust::Output<
            super::super::types::networkfunction::CollectorPolicyIpfxIngestion,
        >,
        /// Specifies the Azure Region where the Network Function Collector Policy should exist. Changing this forces a new Network Function Collector Policy to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the name which should be used for this Network Function Collector Policy. Changing this forces a new Network Function Collector Policy to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// A mapping of tags which should be assigned to the Network Function Collector Policy.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Azure Traffic Collector ID of the Network Function Collector Policy. Changing this forces a new Network Function Collector Policy to be created.
        #[builder(into)]
        pub traffic_collector_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct CollectorPolicyResult {
        /// An `ipfx_emission` block as defined below. Changing this forces a new Network Function Collector Policy to be created.
        pub ipfx_emission: pulumi_wasm_rust::Output<
            super::super::types::networkfunction::CollectorPolicyIpfxEmission,
        >,
        /// An `ipfx_ingestion` block as defined below. Changing this forces a new Network Function Collector Policy to be created.
        pub ipfx_ingestion: pulumi_wasm_rust::Output<
            super::super::types::networkfunction::CollectorPolicyIpfxIngestion,
        >,
        /// Specifies the Azure Region where the Network Function Collector Policy should exist. Changing this forces a new Network Function Collector Policy to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name which should be used for this Network Function Collector Policy. Changing this forces a new Network Function Collector Policy to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Network Function Collector Policy.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Azure Traffic Collector ID of the Network Function Collector Policy. Changing this forces a new Network Function Collector Policy to be created.
        pub traffic_collector_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: CollectorPolicyArgs) -> CollectorPolicyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let ipfx_emission_binding = args.ipfx_emission.get_inner();
        let ipfx_ingestion_binding = args.ipfx_ingestion.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let traffic_collector_id_binding = args.traffic_collector_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:networkfunction/collectorPolicy:CollectorPolicy".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "ipfxEmission".into(),
                    value: &ipfx_emission_binding,
                },
                register_interface::ObjectField {
                    name: "ipfxIngestion".into(),
                    value: &ipfx_ingestion_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "trafficCollectorId".into(),
                    value: &traffic_collector_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "ipfxEmission".into(),
                },
                register_interface::ResultField {
                    name: "ipfxIngestion".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "trafficCollectorId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        CollectorPolicyResult {
            ipfx_emission: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipfxEmission").unwrap(),
            ),
            ipfx_ingestion: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ipfxIngestion").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            traffic_collector_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trafficCollectorId").unwrap(),
            ),
        }
    }
}