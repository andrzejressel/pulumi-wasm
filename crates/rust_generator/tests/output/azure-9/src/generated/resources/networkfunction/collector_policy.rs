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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod collector_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CollectorPolicyArgs {
        /// An `ipfx_emission` block as defined below. Changing this forces a new Network Function Collector Policy to be created.
        #[builder(into)]
        pub ipfx_emission: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::networkfunction::CollectorPolicyIpfxEmission,
        >,
        /// An `ipfx_ingestion` block as defined below. Changing this forces a new Network Function Collector Policy to be created.
        #[builder(into)]
        pub ipfx_ingestion: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::networkfunction::CollectorPolicyIpfxIngestion,
        >,
        /// Specifies the Azure Region where the Network Function Collector Policy should exist. Changing this forces a new Network Function Collector Policy to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name which should be used for this Network Function Collector Policy. Changing this forces a new Network Function Collector Policy to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A mapping of tags which should be assigned to the Network Function Collector Policy.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Azure Traffic Collector ID of the Network Function Collector Policy. Changing this forces a new Network Function Collector Policy to be created.
        #[builder(into)]
        pub traffic_collector_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CollectorPolicyResult {
        /// An `ipfx_emission` block as defined below. Changing this forces a new Network Function Collector Policy to be created.
        pub ipfx_emission: pulumi_gestalt_rust::Output<
            super::super::types::networkfunction::CollectorPolicyIpfxEmission,
        >,
        /// An `ipfx_ingestion` block as defined below. Changing this forces a new Network Function Collector Policy to be created.
        pub ipfx_ingestion: pulumi_gestalt_rust::Output<
            super::super::types::networkfunction::CollectorPolicyIpfxIngestion,
        >,
        /// Specifies the Azure Region where the Network Function Collector Policy should exist. Changing this forces a new Network Function Collector Policy to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name which should be used for this Network Function Collector Policy. Changing this forces a new Network Function Collector Policy to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Network Function Collector Policy.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the Azure Traffic Collector ID of the Network Function Collector Policy. Changing this forces a new Network Function Collector Policy to be created.
        pub traffic_collector_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: CollectorPolicyArgs,
    ) -> CollectorPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let ipfx_emission_binding = args.ipfx_emission.get_output(context);
        let ipfx_ingestion_binding = args.ipfx_ingestion.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let traffic_collector_id_binding = args.traffic_collector_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:networkfunction/collectorPolicy:CollectorPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipfxEmission".into(),
                    value: ipfx_emission_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipfxIngestion".into(),
                    value: ipfx_ingestion_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "trafficCollectorId".into(),
                    value: traffic_collector_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        CollectorPolicyResult {
            ipfx_emission: o.get_field("ipfxEmission"),
            ipfx_ingestion: o.get_field("ipfxIngestion"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            traffic_collector_id: o.get_field("trafficCollectorId"),
        }
    }
}
