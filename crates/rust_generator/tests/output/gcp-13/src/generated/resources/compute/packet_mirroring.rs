/// Packet Mirroring mirrors traffic to and from particular VM instances.
/// You can use the collected traffic to help you detect security threats
/// and monitor application performance.
///
///
/// To get more information about PacketMirroring, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/v1/packetMirrorings)
/// * How-to Guides
///     * [Using Packet Mirroring](https://cloud.google.com/vpc/docs/using-packet-mirroring#creating)
///
/// ## Example Usage
///
/// ### Compute Packet Mirroring Full
///
///
/// ```yaml
/// resources:
///   mirror:
///     type: gcp:compute:Instance
///     properties:
///       networkInterfaces:
///         - accessConfigs:
///             - {}
///           network: ${default.id}
///       name: my-instance
///       machineType: e2-medium
///       bootDisk:
///         initializeParams:
///           image: debian-cloud/debian-11
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: my-network
///   defaultSubnetwork:
///     type: gcp:compute:Subnetwork
///     name: default
///     properties:
///       name: my-subnetwork
///       network: ${default.id}
///       ipCidrRange: 10.2.0.0/16
///   defaultRegionBackendService:
///     type: gcp:compute:RegionBackendService
///     name: default
///     properties:
///       name: my-service
///       healthChecks: ${defaultHealthCheck.id}
///   defaultHealthCheck:
///     type: gcp:compute:HealthCheck
///     name: default
///     properties:
///       name: my-healthcheck
///       checkIntervalSec: 1
///       timeoutSec: 1
///       tcpHealthCheck:
///         port: '80'
///   defaultForwardingRule:
///     type: gcp:compute:ForwardingRule
///     name: default
///     properties:
///       name: my-ilb
///       isMirroringCollector: true
///       ipProtocol: TCP
///       loadBalancingScheme: INTERNAL
///       backendService: ${defaultRegionBackendService.id}
///       allPorts: true
///       network: ${default.id}
///       subnetwork: ${defaultSubnetwork.id}
///       networkTier: PREMIUM
///     options:
///       dependsOn:
///         - ${defaultSubnetwork}
///   foobar:
///     type: gcp:compute:PacketMirroring
///     properties:
///       name: my-mirroring
///       description: bar
///       network:
///         url: ${default.id}
///       collectorIlb:
///         url: ${defaultForwardingRule.id}
///       mirroredResources:
///         tags:
///           - foo
///         instances:
///           - url: ${mirror.id}
///       filter:
///         ipProtocols:
///           - tcp
///         cidrRanges:
///           - 0.0.0.0/0
///         direction: BOTH
/// ```
///
/// ## Import
///
/// PacketMirroring can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/regions/{{region}}/packetMirrorings/{{name}}`
///
/// * `{{project}}/{{region}}/{{name}}`
///
/// * `{{region}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, PacketMirroring can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/packetMirroring:PacketMirroring default projects/{{project}}/regions/{{region}}/packetMirrorings/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/packetMirroring:PacketMirroring default {{project}}/{{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/packetMirroring:PacketMirroring default {{region}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/packetMirroring:PacketMirroring default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod packet_mirroring {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PacketMirroringArgs {
        /// The Forwarding Rule resource (of type load_balancing_scheme=INTERNAL)
        /// that will be used as collector for mirrored traffic. The
        /// specified forwarding rule must have is_mirroring_collector
        /// set to true.
        /// Structure is documented below.
        #[builder(into)]
        pub collector_ilb: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::compute::PacketMirroringCollectorIlb,
        >,
        /// A human-readable description of the rule.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A filter for mirrored traffic. If unset, all traffic is mirrored.
        #[builder(into, default)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::compute::PacketMirroringFilter>,
        >,
        /// A means of specifying which resources to mirror.
        /// Structure is documented below.
        #[builder(into)]
        pub mirrored_resources: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::compute::PacketMirroringMirroredResources,
        >,
        /// The name of the packet mirroring rule
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the mirrored VPC network. Only packets in this network
        /// will be mirrored. All mirrored VMs should have a NIC in the given
        /// network. All mirrored subnetworks should belong to the given network.
        /// Structure is documented below.
        #[builder(into)]
        pub network: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::compute::PacketMirroringNetwork,
        >,
        /// Since only one rule can be active at a time, priority is used to break ties in the case of two rules that apply to the
        /// same instances.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Region in which the created address should reside. If it is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct PacketMirroringResult {
        /// The Forwarding Rule resource (of type load_balancing_scheme=INTERNAL)
        /// that will be used as collector for mirrored traffic. The
        /// specified forwarding rule must have is_mirroring_collector
        /// set to true.
        /// Structure is documented below.
        pub collector_ilb: pulumi_gestalt_rust::Output<
            super::super::types::compute::PacketMirroringCollectorIlb,
        >,
        /// A human-readable description of the rule.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A filter for mirrored traffic. If unset, all traffic is mirrored.
        pub filter: pulumi_gestalt_rust::Output<
            Option<super::super::types::compute::PacketMirroringFilter>,
        >,
        /// A means of specifying which resources to mirror.
        /// Structure is documented below.
        pub mirrored_resources: pulumi_gestalt_rust::Output<
            super::super::types::compute::PacketMirroringMirroredResources,
        >,
        /// The name of the packet mirroring rule
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the mirrored VPC network. Only packets in this network
        /// will be mirrored. All mirrored VMs should have a NIC in the given
        /// network. All mirrored subnetworks should belong to the given network.
        /// Structure is documented below.
        pub network: pulumi_gestalt_rust::Output<
            super::super::types::compute::PacketMirroringNetwork,
        >,
        /// Since only one rule can be active at a time, priority is used to break ties in the case of two rules that apply to the
        /// same instances.
        pub priority: pulumi_gestalt_rust::Output<i32>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The Region in which the created address should reside. If it is not provided, the provider region is used.
        pub region: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PacketMirroringArgs,
    ) -> PacketMirroringResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let collector_ilb_binding = args.collector_ilb.get_output(context);
        let description_binding = args.description.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let mirrored_resources_binding = args.mirrored_resources.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_binding = args.network.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/packetMirroring:PacketMirroring".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "collectorIlb".into(),
                    value: collector_ilb_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: filter_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mirroredResources".into(),
                    value: mirrored_resources_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: network_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: priority_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "region".into(),
                    value: region_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        PacketMirroringResult {
            collector_ilb: o.get_field("collectorIlb"),
            description: o.get_field("description"),
            filter: o.get_field("filter"),
            mirrored_resources: o.get_field("mirroredResources"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            priority: o.get_field("priority"),
            project: o.get_field("project"),
            region: o.get_field("region"),
        }
    }
}
