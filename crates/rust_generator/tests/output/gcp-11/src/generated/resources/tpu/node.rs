/// A Cloud TPU instance.
///
///
/// To get more information about Node, see:
///
/// * [API documentation](https://cloud.google.com/tpu/docs/reference/rest/v1/projects.locations.nodes)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/tpu/docs/)
///
/// ## Example Usage
///
/// ### Tpu Node Basic
///
///
/// ```yaml
/// resources:
///   tpu:
///     type: gcp:tpu:Node
///     properties:
///       name: test-tpu
///       zone: us-central1-b
///       acceleratorType: v3-8
///       tensorflowVersion: ${available.versions[0]}
///       cidrBlock: 10.2.0.0/29
/// variables:
///   available:
///     fn::invoke:
///       function: gcp:tpu:getTensorflowVersions
///       arguments: {}
/// ```
/// ### Tpu Node Full
///
///
/// ```yaml
/// resources:
///   tpu:
///     type: gcp:tpu:Node
///     properties:
///       name: test-tpu
///       zone: us-central1-b
///       acceleratorType: v3-8
///       tensorflowVersion: ${available.versions[0]}
///       description: Google Provider test TPU
///       useServiceNetworking: true
///       network: ${privateServiceConnection.network}
///       labels:
///         foo: bar
///       schedulingConfig:
///         preemptible: true
///   network:
///     type: gcp:compute:Network
///     properties:
///       name: tpu-node-network
///   serviceRange:
///     type: gcp:compute:GlobalAddress
///     name: service_range
///     properties:
///       name: my-global-address
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 16
///       network: ${network.id}
///   privateServiceConnection:
///     type: gcp:servicenetworking:Connection
///     name: private_service_connection
///     properties:
///       network: ${network.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${serviceRange.name}
/// variables:
///   available:
///     fn::invoke:
///       function: gcp:tpu:getTensorflowVersions
///       arguments: {}
/// ```
///
/// ## Import
///
/// Node can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{zone}}/nodes/{{name}}`
///
/// * `{{project}}/{{zone}}/{{name}}`
///
/// * `{{zone}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Node can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:tpu/node:Node default projects/{{project}}/locations/{{zone}}/nodes/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:tpu/node:Node default {{project}}/{{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:tpu/node:Node default {{zone}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:tpu/node:Node default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod node {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NodeArgs {
        /// The type of hardware accelerators associated with this node.
        #[builder(into)]
        pub accelerator_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The CIDR block that the TPU node will use when selecting an IP
        /// address. This CIDR block must be a /29 block; the Compute Engine
        /// networks API forbids a smaller block, and using a larger block would
        /// be wasteful (a node can only consume one IP address).
        /// Errors will occur if the CIDR block has already been used for a
        /// currently existing TPU node, the CIDR block conflicts with any
        /// subnetworks in the user's provided network, or the provided network
        /// is peered with another network that is using that CIDR block.
        #[builder(into, default)]
        pub cidr_block: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The user-supplied description of the TPU. Maximum of 512 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Resource labels to represent user provided metadata.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The immutable name of the TPU.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of a network to peer the TPU node to. It must be a
        /// preexisting Compute Engine network inside of the project on which
        /// this API has been activated. If none is provided, "default" will be
        /// used.
        #[builder(into, default)]
        pub network: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Sets the scheduling options for this TPU instance.
        /// Structure is documented below.
        #[builder(into, default)]
        pub scheduling_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::tpu::NodeSchedulingConfig>,
        >,
        /// The version of Tensorflow running in the Node.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub tensorflow_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether the VPC peering for the node is set up through Service Networking API.
        /// The VPC Peering should be set up before provisioning the node. If this field is set,
        /// cidr_block field should not be specified. If the network that you want to peer the
        /// TPU Node to is a Shared VPC network, the node must be created with this this field enabled.
        #[builder(into, default)]
        pub use_service_networking: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The GCP location for the TPU. If it is not provided, the provider zone is used.
        #[builder(into, default)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NodeResult {
        /// The type of hardware accelerators associated with this node.
        pub accelerator_type: pulumi_gestalt_rust::Output<String>,
        /// The CIDR block that the TPU node will use when selecting an IP
        /// address. This CIDR block must be a /29 block; the Compute Engine
        /// networks API forbids a smaller block, and using a larger block would
        /// be wasteful (a node can only consume one IP address).
        /// Errors will occur if the CIDR block has already been used for a
        /// currently existing TPU node, the CIDR block conflicts with any
        /// subnetworks in the user's provided network, or the provided network
        /// is peered with another network that is using that CIDR block.
        pub cidr_block: pulumi_gestalt_rust::Output<String>,
        /// The user-supplied description of the TPU. Maximum of 512 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Resource labels to represent user provided metadata.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The immutable name of the TPU.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of a network to peer the TPU node to. It must be a
        /// preexisting Compute Engine network inside of the project on which
        /// this API has been activated. If none is provided, "default" will be
        /// used.
        pub network: pulumi_gestalt_rust::Output<String>,
        /// The network endpoints where TPU workers can be accessed and sent work.
        /// It is recommended that Tensorflow clients of the node first reach out
        /// to the first (index 0) entry.
        /// Structure is documented below.
        pub network_endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::types::tpu::NodeNetworkEndpoint>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Sets the scheduling options for this TPU instance.
        /// Structure is documented below.
        pub scheduling_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::tpu::NodeSchedulingConfig>,
        >,
        /// The service account used to run the tensor flow services within the
        /// node. To share resources, including Google Cloud Storage data, with
        /// the Tensorflow job running in the Node, this account must have
        /// permissions to that data.
        pub service_account: pulumi_gestalt_rust::Output<String>,
        /// The version of Tensorflow running in the Node.
        ///
        ///
        /// - - -
        pub tensorflow_version: pulumi_gestalt_rust::Output<String>,
        /// Whether the VPC peering for the node is set up through Service Networking API.
        /// The VPC Peering should be set up before provisioning the node. If this field is set,
        /// cidr_block field should not be specified. If the network that you want to peer the
        /// TPU Node to is a Shared VPC network, the node must be created with this this field enabled.
        pub use_service_networking: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The GCP location for the TPU. If it is not provided, the provider zone is used.
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NodeArgs,
    ) -> NodeResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let accelerator_type_binding = args.accelerator_type.get_output(context);
        let cidr_block_binding = args.cidr_block.get_output(context);
        let description_binding = args.description.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let name_binding = args.name.get_output(context);
        let network_binding = args.network.get_output(context);
        let project_binding = args.project.get_output(context);
        let scheduling_config_binding = args.scheduling_config.get_output(context);
        let tensorflow_version_binding = args.tensorflow_version.get_output(context);
        let use_service_networking_binding = args
            .use_service_networking
            .get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:tpu/node:Node".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "acceleratorType".into(),
                    value: &accelerator_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cidrBlock".into(),
                    value: &cidr_block_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "network".into(),
                    value: &network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "schedulingConfig".into(),
                    value: &scheduling_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tensorflowVersion".into(),
                    value: &tensorflow_version_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "useServiceNetworking".into(),
                    value: &use_service_networking_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: &zone_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        NodeResult {
            accelerator_type: o.get_field("acceleratorType"),
            cidr_block: o.get_field("cidrBlock"),
            description: o.get_field("description"),
            effective_labels: o.get_field("effectiveLabels"),
            labels: o.get_field("labels"),
            name: o.get_field("name"),
            network: o.get_field("network"),
            network_endpoints: o.get_field("networkEndpoints"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            scheduling_config: o.get_field("schedulingConfig"),
            service_account: o.get_field("serviceAccount"),
            tensorflow_version: o.get_field("tensorflowVersion"),
            use_service_networking: o.get_field("useServiceNetworking"),
            zone: o.get_field("zone"),
        }
    }
}
