/// Provides an Neptune Cluster Endpoint Resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cluster_endpoint::create(
///         "example",
///         ClusterEndpointArgs::builder()
///             .cluster_endpoint_identifier("example")
///             .cluster_identifier("${test.clusterIdentifier}")
///             .endpoint_type("READER")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_neptune_cluster_endpoint` using the `cluster-identifier:endpoint-identfier`. For example:
///
/// ```sh
/// $ pulumi import aws:neptune/clusterEndpoint:ClusterEndpoint example my-cluster:my-endpoint
/// ```
pub mod cluster_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterEndpointArgs {
        /// The identifier of the endpoint.
        #[builder(into)]
        pub cluster_endpoint_identifier: pulumi_wasm_rust::Output<String>,
        /// The DB cluster identifier of the DB cluster associated with the endpoint.
        #[builder(into)]
        pub cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// The type of the endpoint. One of: `READER`, `WRITER`, `ANY`.
        #[builder(into)]
        pub endpoint_type: pulumi_wasm_rust::Output<String>,
        /// List of DB instance identifiers that aren't part of the custom endpoint group. All other eligible instances are reachable through the custom endpoint. Only relevant if the list of static members is empty.
        #[builder(into, default)]
        pub excluded_members: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of DB instance identifiers that are part of the custom endpoint group.
        #[builder(into, default)]
        pub static_members: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A map of tags to assign to the Neptune cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterEndpointResult {
        /// The Neptune Cluster Endpoint Amazon Resource Name (ARN).
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The identifier of the endpoint.
        pub cluster_endpoint_identifier: pulumi_wasm_rust::Output<String>,
        /// The DB cluster identifier of the DB cluster associated with the endpoint.
        pub cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// The DNS address of the endpoint.
        pub endpoint: pulumi_wasm_rust::Output<String>,
        /// The type of the endpoint. One of: `READER`, `WRITER`, `ANY`.
        pub endpoint_type: pulumi_wasm_rust::Output<String>,
        /// List of DB instance identifiers that aren't part of the custom endpoint group. All other eligible instances are reachable through the custom endpoint. Only relevant if the list of static members is empty.
        pub excluded_members: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of DB instance identifiers that are part of the custom endpoint group.
        pub static_members: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// A map of tags to assign to the Neptune cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ClusterEndpointArgs) -> ClusterEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_endpoint_identifier_binding = args
            .cluster_endpoint_identifier
            .get_inner();
        let cluster_identifier_binding = args.cluster_identifier.get_inner();
        let endpoint_type_binding = args.endpoint_type.get_inner();
        let excluded_members_binding = args.excluded_members.get_inner();
        let static_members_binding = args.static_members.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:neptune/clusterEndpoint:ClusterEndpoint".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterEndpointIdentifier".into(),
                    value: &cluster_endpoint_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "endpointType".into(),
                    value: &endpoint_type_binding,
                },
                register_interface::ObjectField {
                    name: "excludedMembers".into(),
                    value: &excluded_members_binding,
                },
                register_interface::ObjectField {
                    name: "staticMembers".into(),
                    value: &static_members_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "clusterEndpointIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "clusterIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "endpoint".into(),
                },
                register_interface::ResultField {
                    name: "endpointType".into(),
                },
                register_interface::ResultField {
                    name: "excludedMembers".into(),
                },
                register_interface::ResultField {
                    name: "staticMembers".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ClusterEndpointResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            cluster_endpoint_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterEndpointIdentifier").unwrap(),
            ),
            cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("clusterIdentifier").unwrap(),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpoint").unwrap(),
            ),
            endpoint_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("endpointType").unwrap(),
            ),
            excluded_members: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("excludedMembers").unwrap(),
            ),
            static_members: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("staticMembers").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}