/// Provides an Neptune Cluster Endpoint Resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterEndpointArgs {
        /// The identifier of the endpoint.
        #[builder(into)]
        pub cluster_endpoint_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The DB cluster identifier of the DB cluster associated with the endpoint.
        #[builder(into)]
        pub cluster_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of the endpoint. One of: `READER`, `WRITER`, `ANY`.
        #[builder(into)]
        pub endpoint_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of DB instance identifiers that aren't part of the custom endpoint group. All other eligible instances are reachable through the custom endpoint. Only relevant if the list of static members is empty.
        #[builder(into, default)]
        pub excluded_members: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of DB instance identifiers that are part of the custom endpoint group.
        #[builder(into, default)]
        pub static_members: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// A map of tags to assign to the Neptune cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterEndpointResult {
        /// The Neptune Cluster Endpoint Amazon Resource Name (ARN).
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The identifier of the endpoint.
        pub cluster_endpoint_identifier: pulumi_gestalt_rust::Output<String>,
        /// The DB cluster identifier of the DB cluster associated with the endpoint.
        pub cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// The DNS address of the endpoint.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// The type of the endpoint. One of: `READER`, `WRITER`, `ANY`.
        pub endpoint_type: pulumi_gestalt_rust::Output<String>,
        /// List of DB instance identifiers that aren't part of the custom endpoint group. All other eligible instances are reachable through the custom endpoint. Only relevant if the list of static members is empty.
        pub excluded_members: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// List of DB instance identifiers that are part of the custom endpoint group.
        pub static_members: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// A map of tags to assign to the Neptune cluster. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ClusterEndpointArgs,
    ) -> ClusterEndpointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cluster_endpoint_identifier_binding = args
            .cluster_endpoint_identifier
            .get_output(context);
        let cluster_identifier_binding = args.cluster_identifier.get_output(context);
        let endpoint_type_binding = args.endpoint_type.get_output(context);
        let excluded_members_binding = args.excluded_members.get_output(context);
        let static_members_binding = args.static_members.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:neptune/clusterEndpoint:ClusterEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterEndpointIdentifier".into(),
                    value: cluster_endpoint_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterIdentifier".into(),
                    value: cluster_identifier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointType".into(),
                    value: endpoint_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "excludedMembers".into(),
                    value: excluded_members_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "staticMembers".into(),
                    value: static_members_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterEndpointResult {
            arn: o.get_field("arn"),
            cluster_endpoint_identifier: o.get_field("clusterEndpointIdentifier"),
            cluster_identifier: o.get_field("clusterIdentifier"),
            endpoint: o.get_field("endpoint"),
            endpoint_type: o.get_field("endpointType"),
            excluded_members: o.get_field("excludedMembers"),
            static_members: o.get_field("staticMembers"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
