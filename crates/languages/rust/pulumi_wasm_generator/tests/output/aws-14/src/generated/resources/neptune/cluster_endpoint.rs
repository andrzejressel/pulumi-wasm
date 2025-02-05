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
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterEndpointArgs {
        /// The identifier of the endpoint.
        #[builder(into)]
        pub cluster_endpoint_identifier: pulumi_wasm_rust::InputOrOutput<String>,
        /// The DB cluster identifier of the DB cluster associated with the endpoint.
        #[builder(into)]
        pub cluster_identifier: pulumi_wasm_rust::InputOrOutput<String>,
        /// The type of the endpoint. One of: `READER`, `WRITER`, `ANY`.
        #[builder(into)]
        pub endpoint_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// List of DB instance identifiers that aren't part of the custom endpoint group. All other eligible instances are reachable through the custom endpoint. Only relevant if the list of static members is empty.
        #[builder(into, default)]
        pub excluded_members: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of DB instance identifiers that are part of the custom endpoint group.
        #[builder(into, default)]
        pub static_members: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// A map of tags to assign to the Neptune cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
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
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ClusterEndpointArgs,
    ) -> ClusterEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_endpoint_identifier_binding = args
            .cluster_endpoint_identifier
            .get_output(context)
            .get_inner();
        let cluster_identifier_binding = args
            .cluster_identifier
            .get_output(context)
            .get_inner();
        let endpoint_type_binding = args.endpoint_type.get_output(context).get_inner();
        let excluded_members_binding = args
            .excluded_members
            .get_output(context)
            .get_inner();
        let static_members_binding = args.static_members.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:neptune/clusterEndpoint:ClusterEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ClusterEndpointResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            cluster_endpoint_identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterEndpointIdentifier"),
            ),
            cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterIdentifier"),
            ),
            endpoint: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            endpoint_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("endpointType"),
            ),
            excluded_members: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("excludedMembers"),
            ),
            static_members: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("staticMembers"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}
