/// Manages an RDS Aurora Cluster Custom Endpoint.
/// You can refer to the [User Guide](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/Aurora.Overview.Endpoints.html#Aurora.Endpoints.Cluster).
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   default:
///     type: aws:rds:Cluster
///     properties:
///       clusterIdentifier: aurora-cluster-demo
///       availabilityZones:
///         - us-west-2a
///         - us-west-2b
///         - us-west-2c
///       databaseName: mydb
///       masterUsername: foo
///       masterPassword: bar
///       backupRetentionPeriod: 5
///       preferredBackupWindow: 07:00-09:00
///   test1:
///     type: aws:rds:ClusterInstance
///     properties:
///       applyImmediately: true
///       clusterIdentifier: ${default.id}
///       identifier: test1
///       instanceClass: db.t2.small
///       engine: ${default.engine}
///       engineVersion: ${default.engineVersion}
///   test2:
///     type: aws:rds:ClusterInstance
///     properties:
///       applyImmediately: true
///       clusterIdentifier: ${default.id}
///       identifier: test2
///       instanceClass: db.t2.small
///       engine: ${default.engine}
///       engineVersion: ${default.engineVersion}
///   test3:
///     type: aws:rds:ClusterInstance
///     properties:
///       applyImmediately: true
///       clusterIdentifier: ${default.id}
///       identifier: test3
///       instanceClass: db.t2.small
///       engine: ${default.engine}
///       engineVersion: ${default.engineVersion}
///   eligible:
///     type: aws:rds:ClusterEndpoint
///     properties:
///       clusterIdentifier: ${default.id}
///       clusterEndpointIdentifier: reader
///       customEndpointType: READER
///       excludedMembers:
///         - ${test1.id}
///         - ${test2.id}
///   static:
///     type: aws:rds:ClusterEndpoint
///     properties:
///       clusterIdentifier: ${default.id}
///       clusterEndpointIdentifier: static
///       customEndpointType: READER
///       staticMembers:
///         - ${test1.id}
///         - ${test3.id}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import RDS Clusters Endpoint using the `cluster_endpoint_identifier`. For example:
///
/// ```sh
/// $ pulumi import aws:rds/clusterEndpoint:ClusterEndpoint custom_reader aurora-prod-cluster-custom-reader
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod cluster_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterEndpointArgs {
        /// The identifier to use for the new endpoint. This parameter is stored as a lowercase string.
        #[builder(into)]
        pub cluster_endpoint_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The cluster identifier.
        #[builder(into)]
        pub cluster_identifier: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of the endpoint. One of: READER , ANY .
        #[builder(into)]
        pub custom_endpoint_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of DB instance identifiers that aren't part of the custom endpoint group. All other eligible instances are reachable through the custom endpoint. Only relevant if the list of static members is empty. Conflicts with `static_members`.
        #[builder(into, default)]
        pub excluded_members: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of DB instance identifiers that are part of the custom endpoint group. Conflicts with `excluded_members`.
        #[builder(into, default)]
        pub static_members: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ClusterEndpointResult {
        /// Amazon Resource Name (ARN) of cluster
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The identifier to use for the new endpoint. This parameter is stored as a lowercase string.
        pub cluster_endpoint_identifier: pulumi_gestalt_rust::Output<String>,
        /// The cluster identifier.
        pub cluster_identifier: pulumi_gestalt_rust::Output<String>,
        /// The type of the endpoint. One of: READER , ANY .
        pub custom_endpoint_type: pulumi_gestalt_rust::Output<String>,
        /// A custom endpoint for the Aurora cluster
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// List of DB instance identifiers that aren't part of the custom endpoint group. All other eligible instances are reachable through the custom endpoint. Only relevant if the list of static members is empty. Conflicts with `static_members`.
        pub excluded_members: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// List of DB instance identifiers that are part of the custom endpoint group. Conflicts with `excluded_members`.
        pub static_members: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        let custom_endpoint_type_binding = args.custom_endpoint_type.get_output(context);
        let excluded_members_binding = args.excluded_members.get_output(context);
        let static_members_binding = args.static_members.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:rds/clusterEndpoint:ClusterEndpoint".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterEndpointIdentifier".into(),
                    value: &cluster_endpoint_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clusterIdentifier".into(),
                    value: &cluster_identifier_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "customEndpointType".into(),
                    value: &custom_endpoint_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "excludedMembers".into(),
                    value: &excluded_members_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "staticMembers".into(),
                    value: &static_members_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ClusterEndpointResult {
            arn: o.get_field("arn"),
            cluster_endpoint_identifier: o.get_field("clusterEndpointIdentifier"),
            cluster_identifier: o.get_field("clusterIdentifier"),
            custom_endpoint_type: o.get_field("customEndpointType"),
            endpoint: o.get_field("endpoint"),
            excluded_members: o.get_field("excludedMembers"),
            static_members: o.get_field("staticMembers"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
