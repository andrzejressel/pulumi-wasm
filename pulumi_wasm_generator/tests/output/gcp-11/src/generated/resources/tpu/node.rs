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
pub mod node {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NodeArgs {
        /// The type of hardware accelerators associated with this node.
        #[builder(into)]
        pub accelerator_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// The CIDR block that the TPU node will use when selecting an IP
        /// address. This CIDR block must be a /29 block; the Compute Engine
        /// networks API forbids a smaller block, and using a larger block would
        /// be wasteful (a node can only consume one IP address).
        /// Errors will occur if the CIDR block has already been used for a
        /// currently existing TPU node, the CIDR block conflicts with any
        /// subnetworks in the user's provided network, or the provided network
        /// is peered with another network that is using that CIDR block.
        #[builder(into, default)]
        pub cidr_block: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The user-supplied description of the TPU. Maximum of 512 characters.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Resource labels to represent user provided metadata.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The immutable name of the TPU.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of a network to peer the TPU node to. It must be a
        /// preexisting Compute Engine network inside of the project on which
        /// this API has been activated. If none is provided, "default" will be
        /// used.
        #[builder(into, default)]
        pub network: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Sets the scheduling options for this TPU instance.
        /// Structure is documented below.
        #[builder(into, default)]
        pub scheduling_config: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::tpu::NodeSchedulingConfig>,
        >,
        /// The version of Tensorflow running in the Node.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub tensorflow_version: pulumi_wasm_rust::InputOrOutput<String>,
        /// Whether the VPC peering for the node is set up through Service Networking API.
        /// The VPC Peering should be set up before provisioning the node. If this field is set,
        /// cidr_block field should not be specified. If the network that you want to peer the
        /// TPU Node to is a Shared VPC network, the node must be created with this this field enabled.
        #[builder(into, default)]
        pub use_service_networking: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The GCP location for the TPU. If it is not provided, the provider zone is used.
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct NodeResult {
        /// The type of hardware accelerators associated with this node.
        pub accelerator_type: pulumi_wasm_rust::Output<String>,
        /// The CIDR block that the TPU node will use when selecting an IP
        /// address. This CIDR block must be a /29 block; the Compute Engine
        /// networks API forbids a smaller block, and using a larger block would
        /// be wasteful (a node can only consume one IP address).
        /// Errors will occur if the CIDR block has already been used for a
        /// currently existing TPU node, the CIDR block conflicts with any
        /// subnetworks in the user's provided network, or the provided network
        /// is peered with another network that is using that CIDR block.
        pub cidr_block: pulumi_wasm_rust::Output<String>,
        /// The user-supplied description of the TPU. Maximum of 512 characters.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Resource labels to represent user provided metadata.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The immutable name of the TPU.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of a network to peer the TPU node to. It must be a
        /// preexisting Compute Engine network inside of the project on which
        /// this API has been activated. If none is provided, "default" will be
        /// used.
        pub network: pulumi_wasm_rust::Output<String>,
        /// The network endpoints where TPU workers can be accessed and sent work.
        /// It is recommended that Tensorflow clients of the node first reach out
        /// to the first (index 0) entry.
        /// Structure is documented below.
        pub network_endpoints: pulumi_wasm_rust::Output<
            Vec<super::super::types::tpu::NodeNetworkEndpoint>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Sets the scheduling options for this TPU instance.
        /// Structure is documented below.
        pub scheduling_config: pulumi_wasm_rust::Output<
            Option<super::super::types::tpu::NodeSchedulingConfig>,
        >,
        /// The service account used to run the tensor flow services within the
        /// node. To share resources, including Google Cloud Storage data, with
        /// the Tensorflow job running in the Node, this account must have
        /// permissions to that data.
        pub service_account: pulumi_wasm_rust::Output<String>,
        /// The version of Tensorflow running in the Node.
        ///
        ///
        /// - - -
        pub tensorflow_version: pulumi_wasm_rust::Output<String>,
        /// Whether the VPC peering for the node is set up through Service Networking API.
        /// The VPC Peering should be set up before provisioning the node. If this field is set,
        /// cidr_block field should not be specified. If the network that you want to peer the
        /// TPU Node to is a Shared VPC network, the node must be created with this this field enabled.
        pub use_service_networking: pulumi_wasm_rust::Output<Option<bool>>,
        /// The GCP location for the TPU. If it is not provided, the provider zone is used.
        pub zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: NodeArgs,
    ) -> NodeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let accelerator_type_binding = args
            .accelerator_type
            .get_output(context)
            .get_inner();
        let cidr_block_binding = args.cidr_block.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let scheduling_config_binding = args
            .scheduling_config
            .get_output(context)
            .get_inner();
        let tensorflow_version_binding = args
            .tensorflow_version
            .get_output(context)
            .get_inner();
        let use_service_networking_binding = args
            .use_service_networking
            .get_output(context)
            .get_inner();
        let zone_binding = args.zone.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:tpu/node:Node".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "acceleratorType".into(),
                    value: &accelerator_type_binding,
                },
                register_interface::ObjectField {
                    name: "cidrBlock".into(),
                    value: &cidr_block_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "schedulingConfig".into(),
                    value: &scheduling_config_binding,
                },
                register_interface::ObjectField {
                    name: "tensorflowVersion".into(),
                    value: &tensorflow_version_binding,
                },
                register_interface::ObjectField {
                    name: "useServiceNetworking".into(),
                    value: &use_service_networking_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "acceleratorType".into(),
                },
                register_interface::ResultField {
                    name: "cidrBlock".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "network".into(),
                },
                register_interface::ResultField {
                    name: "networkEndpoints".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "schedulingConfig".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccount".into(),
                },
                register_interface::ResultField {
                    name: "tensorflowVersion".into(),
                },
                register_interface::ResultField {
                    name: "useServiceNetworking".into(),
                },
                register_interface::ResultField {
                    name: "zone".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        NodeResult {
            accelerator_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acceleratorType").unwrap(),
            ),
            cidr_block: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cidrBlock").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            network: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("network").unwrap(),
            ),
            network_endpoints: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkEndpoints").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            scheduling_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("schedulingConfig").unwrap(),
            ),
            service_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccount").unwrap(),
            ),
            tensorflow_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tensorflowVersion").unwrap(),
            ),
            use_service_networking: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("useServiceNetworking").unwrap(),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zone").unwrap(),
            ),
        }
    }
}
