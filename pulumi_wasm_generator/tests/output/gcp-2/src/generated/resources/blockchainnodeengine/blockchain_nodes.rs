/// A representation of a blockchain node.
///
///
/// To get more information about BlockchainNodes, see:
///
/// * [API documentation](https://cloud.google.com/blockchain-node-engine/docs/reference/rest/v1/projects.locations.blockchainNodes)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/blockchain-node-engine)
///
/// ## Example Usage
///
/// ### Blockchain Nodes Basic
///
///
/// ```yaml
/// resources:
///   defaultNode:
///     type: gcp:blockchainnodeengine:BlockchainNodes
///     name: default_node
///     properties:
///       location: us-central1
///       blockchainType: ETHEREUM
///       blockchainNodeId: blockchain_basic_node
///       ethereumDetails:
///         apiEnableAdmin: true
///         apiEnableDebug: true
///         validatorConfig:
///           mevRelayUrls:
///             - https://mev1.example.org/
///             - https://mev2.example.org/
///         nodeType: ARCHIVE
///         consensusClient: LIGHTHOUSE
///         executionClient: ERIGON
///         network: MAINNET
///       labels:
///         environment: dev
/// ```
/// ### Blockchain Nodes Geth Details
///
///
/// ```yaml
/// resources:
///   defaultNodeGeth:
///     type: gcp:blockchainnodeengine:BlockchainNodes
///     name: default_node_geth
///     properties:
///       location: us-central1
///       blockchainType: ETHEREUM
///       blockchainNodeId: blockchain_geth_node
///       ethereumDetails:
///         apiEnableAdmin: true
///         apiEnableDebug: true
///         validatorConfig:
///           mevRelayUrls:
///             - https://mev1.example.org/
///             - https://mev2.example.org/
///         nodeType: FULL
///         consensusClient: LIGHTHOUSE
///         executionClient: GETH
///         network: MAINNET
///         gethDetails:
///           garbageCollectionMode: FULL
///       labels:
///         environment: dev
/// ```
///
/// ## Import
///
/// BlockchainNodes can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/blockchainNodes/{{blockchain_node_id}}`
///
/// * `{{project}}/{{location}}/{{blockchain_node_id}}`
///
/// * `{{location}}/{{blockchain_node_id}}`
///
/// When using the `pulumi import` command, BlockchainNodes can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:blockchainnodeengine/blockchainNodes:BlockchainNodes default projects/{{project}}/locations/{{location}}/blockchainNodes/{{blockchain_node_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:blockchainnodeengine/blockchainNodes:BlockchainNodes default {{project}}/{{location}}/{{blockchain_node_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:blockchainnodeengine/blockchainNodes:BlockchainNodes default {{location}}/{{blockchain_node_id}}
/// ```
///
pub mod blockchain_nodes {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BlockchainNodesArgs {
        /// ID of the requesting object.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub blockchain_node_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// User-provided key-value pairs
        /// Possible values are: `ETHEREUM`.
        #[builder(into, default)]
        pub blockchain_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// User-provided key-value pairs
        /// Structure is documented below.
        #[builder(into, default)]
        pub ethereum_details: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::blockchainnodeengine::BlockchainNodesEthereumDetails,
            >,
        >,
        /// User-provided key-value pairs
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location of Blockchain Node being created.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BlockchainNodesResult {
        /// ID of the requesting object.
        ///
        ///
        /// - - -
        pub blockchain_node_id: pulumi_wasm_rust::Output<String>,
        /// User-provided key-value pairs
        /// Possible values are: `ETHEREUM`.
        pub blockchain_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The connection information through which to interact with a blockchain node.
        /// Structure is documented below.
        pub connection_infos: pulumi_wasm_rust::Output<
            Vec<super::super::types::blockchainnodeengine::BlockchainNodesConnectionInfo>,
        >,
        /// The timestamp at which the blockchain node was first created.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// User-provided key-value pairs
        /// Structure is documented below.
        pub ethereum_details: pulumi_wasm_rust::Output<
            Option<
                super::super::types::blockchainnodeengine::BlockchainNodesEthereumDetails,
            >,
        >,
        /// User-provided key-value pairs
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Location of Blockchain Node being created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The fully qualified name of the blockchain node. e.g. projects/my-project/locations/us-central1/blockchainNodes/my-node.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The timestamp at which the blockchain node was last updated.
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: BlockchainNodesArgs,
    ) -> BlockchainNodesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let blockchain_node_id_binding = args
            .blockchain_node_id
            .get_output(context)
            .get_inner();
        let blockchain_type_binding = args
            .blockchain_type
            .get_output(context)
            .get_inner();
        let ethereum_details_binding = args
            .ethereum_details
            .get_output(context)
            .get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:blockchainnodeengine/blockchainNodes:BlockchainNodes".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "blockchainNodeId".into(),
                    value: &blockchain_node_id_binding,
                },
                register_interface::ObjectField {
                    name: "blockchainType".into(),
                    value: &blockchain_type_binding,
                },
                register_interface::ObjectField {
                    name: "ethereumDetails".into(),
                    value: &ethereum_details_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "blockchainNodeId".into(),
                },
                register_interface::ResultField {
                    name: "blockchainType".into(),
                },
                register_interface::ResultField {
                    name: "connectionInfos".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "ethereumDetails".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        BlockchainNodesResult {
            blockchain_node_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blockchainNodeId").unwrap(),
            ),
            blockchain_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blockchainType").unwrap(),
            ),
            connection_infos: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionInfos").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            ethereum_details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ethereumDetails").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}
