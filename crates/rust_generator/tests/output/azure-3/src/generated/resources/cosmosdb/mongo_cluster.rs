/// Manages a MongoDB Cluster using vCore Architecture.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: East US
///   exampleMongoCluster:
///     type: azure:cosmosdb:MongoCluster
///     name: example
///     properties:
///       name: example-mc
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       administratorUsername: adminTerraform
///       administratorPassword: QAZwsx123
///       shardCount: '1'
///       computeTier: Free
///       highAvailabilityMode: Disabled
///       storageSizeInGb: '32'
/// ```
///
///
/// ### Preview Feature GeoReplicas)
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-rg
///       location: East US
///   exampleMongoCluster:
///     type: azure:cosmosdb:MongoCluster
///     name: example
///     properties:
///       name: example-mc
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       administratorUsername: adminTerraform
///       administratorPassword: QAZwsx123
///       shardCount: '1'
///       computeTier: M30
///       highAvailabilityMode: ZoneRedundantPreferred
///       storageSizeInGb: '64'
///       previewFeatures:
///         - GeoReplicas
///   exampleGeoReplica:
///     type: azure:cosmosdb:MongoCluster
///     name: example_geo_replica
///     properties:
///       name: example-mc-geo
///       resourceGroupName: ${example.name}
///       location: Central US
///       sourceServerId: ${exampleMongoCluster.id}
///       sourceLocation: ${exampleMongoCluster.location}
///       createMode: GeoReplica
/// ```
///
/// ## Import
///
/// MongoDB Clusters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cosmosdb/mongoCluster:MongoCluster example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DocumentDB/mongoClusters/myMongoCluster
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod mongo_cluster {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MongoClusterArgs {
        /// The Password associated with the `administrator_username` for the MongoDB Cluster.
        #[builder(into, default)]
        pub administrator_password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The administrator username of the MongoDB Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub administrator_username: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The compute tier to assign to the MongoDB Cluster. Possible values are `Free`, `M25`, `M30`, `M40`, `M50`, `M60` and `M80`.
        #[builder(into, default)]
        pub compute_tier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The creation mode for the MongoDB Cluster. Possibles values are `Default` and `GeoReplica`. Defaults to `Default`. Changing this forces a new resource to be created.
        ///
        /// > **Note** The creation mode `GeoReplica` is currently in preview. It is only available when `preview_features` is set.
        #[builder(into, default)]
        pub create_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The high availability mode for the MongoDB Cluster. Possibles values are `Disabled` and `ZoneRedundantPreferred`.
        #[builder(into, default)]
        pub high_availability_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The supported Azure location where the MongoDB Cluster exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for the MongoDB Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The preview features that can be enabled on the MongoDB Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub preview_features: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The Public Network Access setting for the MongoDB Cluster. Possibles values are `Disabled` and `Enabled`. Defaults to `Enabled`.
        #[builder(into, default)]
        pub public_network_access: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the MongoDB Cluster. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Number of shards to provision on the MongoDB Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub shard_count: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The location of the source MongoDB Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub source_location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the replication source MongoDB Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub source_server_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The size of the data disk space for the MongoDB Cluster.
        #[builder(into, default)]
        pub storage_size_in_gb: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A mapping of tags to assign to the MongoDB Cluster.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The version for the MongoDB Cluster. Possibles values are `5.0`, `6.0` and `7.0`.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MongoClusterResult {
        /// The Password associated with the `administrator_username` for the MongoDB Cluster.
        pub administrator_password: pulumi_gestalt_rust::Output<Option<String>>,
        /// The administrator username of the MongoDB Cluster. Changing this forces a new resource to be created.
        pub administrator_username: pulumi_gestalt_rust::Output<Option<String>>,
        /// The compute tier to assign to the MongoDB Cluster. Possible values are `Free`, `M25`, `M30`, `M40`, `M50`, `M60` and `M80`.
        pub compute_tier: pulumi_gestalt_rust::Output<Option<String>>,
        /// The creation mode for the MongoDB Cluster. Possibles values are `Default` and `GeoReplica`. Defaults to `Default`. Changing this forces a new resource to be created.
        ///
        /// > **Note** The creation mode `GeoReplica` is currently in preview. It is only available when `preview_features` is set.
        pub create_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The high availability mode for the MongoDB Cluster. Possibles values are `Disabled` and `ZoneRedundantPreferred`.
        pub high_availability_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The supported Azure location where the MongoDB Cluster exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for the MongoDB Cluster. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The preview features that can be enabled on the MongoDB Cluster. Changing this forces a new resource to be created.
        pub preview_features: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The Public Network Access setting for the MongoDB Cluster. Possibles values are `Disabled` and `Enabled`. Defaults to `Enabled`.
        pub public_network_access: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the MongoDB Cluster. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The Number of shards to provision on the MongoDB Cluster. Changing this forces a new resource to be created.
        pub shard_count: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The location of the source MongoDB Cluster. Changing this forces a new resource to be created.
        pub source_location: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the replication source MongoDB Cluster. Changing this forces a new resource to be created.
        pub source_server_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The size of the data disk space for the MongoDB Cluster.
        pub storage_size_in_gb: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A mapping of tags to assign to the MongoDB Cluster.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The version for the MongoDB Cluster. Possibles values are `5.0`, `6.0` and `7.0`.
        pub version: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MongoClusterArgs,
    ) -> MongoClusterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let administrator_password_binding = args
            .administrator_password
            .get_output(context);
        let administrator_username_binding = args
            .administrator_username
            .get_output(context);
        let compute_tier_binding = args.compute_tier.get_output(context);
        let create_mode_binding = args.create_mode.get_output(context);
        let high_availability_mode_binding = args
            .high_availability_mode
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let preview_features_binding = args.preview_features.get_output(context);
        let public_network_access_binding = args
            .public_network_access
            .get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let shard_count_binding = args.shard_count.get_output(context);
        let source_location_binding = args.source_location.get_output(context);
        let source_server_id_binding = args.source_server_id.get_output(context);
        let storage_size_in_gb_binding = args.storage_size_in_gb.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cosmosdb/mongoCluster:MongoCluster".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "administratorPassword".into(),
                    value: administrator_password_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "administratorUsername".into(),
                    value: administrator_username_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "computeTier".into(),
                    value: compute_tier_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "createMode".into(),
                    value: create_mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "highAvailabilityMode".into(),
                    value: high_availability_mode_binding.get_id(),
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
                    name: "previewFeatures".into(),
                    value: preview_features_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publicNetworkAccess".into(),
                    value: public_network_access_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "shardCount".into(),
                    value: shard_count_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceLocation".into(),
                    value: source_location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sourceServerId".into(),
                    value: source_server_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageSizeInGb".into(),
                    value: storage_size_in_gb_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: version_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MongoClusterResult {
            administrator_password: o.get_field("administratorPassword"),
            administrator_username: o.get_field("administratorUsername"),
            compute_tier: o.get_field("computeTier"),
            create_mode: o.get_field("createMode"),
            high_availability_mode: o.get_field("highAvailabilityMode"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            preview_features: o.get_field("previewFeatures"),
            public_network_access: o.get_field("publicNetworkAccess"),
            resource_group_name: o.get_field("resourceGroupName"),
            shard_count: o.get_field("shardCount"),
            source_location: o.get_field("sourceLocation"),
            source_server_id: o.get_field("sourceServerId"),
            storage_size_in_gb: o.get_field("storageSizeInGb"),
            tags: o.get_field("tags"),
            version: o.get_field("version"),
        }
    }
}
