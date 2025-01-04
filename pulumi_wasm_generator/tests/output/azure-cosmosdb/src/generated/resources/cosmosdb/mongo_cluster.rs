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
pub mod mongo_cluster {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MongoClusterArgs {
        /// The Password associated with the `administrator_username` for the MongoDB Cluster.
        #[builder(into, default)]
        pub administrator_password: pulumi_wasm_rust::Output<Option<String>>,
        /// The administrator username of the MongoDB Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub administrator_username: pulumi_wasm_rust::Output<Option<String>>,
        /// The compute tier to assign to the MongoDB Cluster. Possible values are `Free`, `M25`, `M30`, `M40`, `M50`, `M60` and `M80`.
        #[builder(into, default)]
        pub compute_tier: pulumi_wasm_rust::Output<Option<String>>,
        /// The creation mode for the MongoDB Cluster. Possibles values are `Default` and `GeoReplica`. Defaults to `Default`. Changing this forces a new resource to be created.
        ///
        /// > **Note** The creation mode `GeoReplica` is currently in preview. It is only available when `preview_features` is set.
        #[builder(into, default)]
        pub create_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The high availability mode for the MongoDB Cluster. Possibles values are `Disabled` and `ZoneRedundantPreferred`.
        #[builder(into, default)]
        pub high_availability_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The supported Azure location where the MongoDB Cluster exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// The name which should be used for the MongoDB Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The preview features that can be enabled on the MongoDB Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub preview_features: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Public Network Access setting for the MongoDB Cluster. Possibles values are `Disabled` and `Enabled`. Defaults to `Enabled`.
        #[builder(into, default)]
        pub public_network_access: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the MongoDB Cluster. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Number of shards to provision on the MongoDB Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub shard_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The location of the source MongoDB Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub source_location: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the replication source MongoDB Cluster. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub source_server_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The size of the data disk space for the MongoDB Cluster.
        #[builder(into, default)]
        pub storage_size_in_gb: pulumi_wasm_rust::Output<Option<i32>>,
        /// A mapping of tags to assign to the MongoDB Cluster.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The version for the MongoDB Cluster. Possibles values are `5.0`, `6.0` and `7.0`.
        #[builder(into, default)]
        pub version: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MongoClusterResult {
        /// The Password associated with the `administrator_username` for the MongoDB Cluster.
        pub administrator_password: pulumi_wasm_rust::Output<Option<String>>,
        /// The administrator username of the MongoDB Cluster. Changing this forces a new resource to be created.
        pub administrator_username: pulumi_wasm_rust::Output<Option<String>>,
        /// The compute tier to assign to the MongoDB Cluster. Possible values are `Free`, `M25`, `M30`, `M40`, `M50`, `M60` and `M80`.
        pub compute_tier: pulumi_wasm_rust::Output<Option<String>>,
        /// The creation mode for the MongoDB Cluster. Possibles values are `Default` and `GeoReplica`. Defaults to `Default`. Changing this forces a new resource to be created.
        ///
        /// > **Note** The creation mode `GeoReplica` is currently in preview. It is only available when `preview_features` is set.
        pub create_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The high availability mode for the MongoDB Cluster. Possibles values are `Disabled` and `ZoneRedundantPreferred`.
        pub high_availability_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The supported Azure location where the MongoDB Cluster exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for the MongoDB Cluster. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The preview features that can be enabled on the MongoDB Cluster. Changing this forces a new resource to be created.
        pub preview_features: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The Public Network Access setting for the MongoDB Cluster. Possibles values are `Disabled` and `Enabled`. Defaults to `Enabled`.
        pub public_network_access: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the resource group in which to create the MongoDB Cluster. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The Number of shards to provision on the MongoDB Cluster. Changing this forces a new resource to be created.
        pub shard_count: pulumi_wasm_rust::Output<Option<i32>>,
        /// The location of the source MongoDB Cluster. Changing this forces a new resource to be created.
        pub source_location: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the replication source MongoDB Cluster. Changing this forces a new resource to be created.
        pub source_server_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The size of the data disk space for the MongoDB Cluster.
        pub storage_size_in_gb: pulumi_wasm_rust::Output<Option<i32>>,
        /// A mapping of tags to assign to the MongoDB Cluster.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The version for the MongoDB Cluster. Possibles values are `5.0`, `6.0` and `7.0`.
        pub version: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: MongoClusterArgs) -> MongoClusterResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let administrator_password_binding = args.administrator_password.get_inner();
        let administrator_username_binding = args.administrator_username.get_inner();
        let compute_tier_binding = args.compute_tier.get_inner();
        let create_mode_binding = args.create_mode.get_inner();
        let high_availability_mode_binding = args.high_availability_mode.get_inner();
        let location_binding = args.location.get_inner();
        let name_binding = args.name.get_inner();
        let preview_features_binding = args.preview_features.get_inner();
        let public_network_access_binding = args.public_network_access.get_inner();
        let resource_group_name_binding = args.resource_group_name.get_inner();
        let shard_count_binding = args.shard_count.get_inner();
        let source_location_binding = args.source_location.get_inner();
        let source_server_id_binding = args.source_server_id.get_inner();
        let storage_size_in_gb_binding = args.storage_size_in_gb.get_inner();
        let tags_binding = args.tags.get_inner();
        let version_binding = args.version.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:cosmosdb/mongoCluster:MongoCluster".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "administratorPassword".into(),
                    value: &administrator_password_binding,
                },
                register_interface::ObjectField {
                    name: "administratorUsername".into(),
                    value: &administrator_username_binding,
                },
                register_interface::ObjectField {
                    name: "computeTier".into(),
                    value: &compute_tier_binding,
                },
                register_interface::ObjectField {
                    name: "createMode".into(),
                    value: &create_mode_binding,
                },
                register_interface::ObjectField {
                    name: "highAvailabilityMode".into(),
                    value: &high_availability_mode_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "previewFeatures".into(),
                    value: &preview_features_binding,
                },
                register_interface::ObjectField {
                    name: "publicNetworkAccess".into(),
                    value: &public_network_access_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "shardCount".into(),
                    value: &shard_count_binding,
                },
                register_interface::ObjectField {
                    name: "sourceLocation".into(),
                    value: &source_location_binding,
                },
                register_interface::ObjectField {
                    name: "sourceServerId".into(),
                    value: &source_server_id_binding,
                },
                register_interface::ObjectField {
                    name: "storageSizeInGb".into(),
                    value: &storage_size_in_gb_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "administratorPassword".into(),
                },
                register_interface::ResultField {
                    name: "administratorUsername".into(),
                },
                register_interface::ResultField {
                    name: "computeTier".into(),
                },
                register_interface::ResultField {
                    name: "createMode".into(),
                },
                register_interface::ResultField {
                    name: "highAvailabilityMode".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "previewFeatures".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccess".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "shardCount".into(),
                },
                register_interface::ResultField {
                    name: "sourceLocation".into(),
                },
                register_interface::ResultField {
                    name: "sourceServerId".into(),
                },
                register_interface::ResultField {
                    name: "storageSizeInGb".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MongoClusterResult {
            administrator_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("administratorPassword").unwrap(),
            ),
            administrator_username: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("administratorUsername").unwrap(),
            ),
            compute_tier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("computeTier").unwrap(),
            ),
            create_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createMode").unwrap(),
            ),
            high_availability_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("highAvailabilityMode").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            preview_features: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("previewFeatures").unwrap(),
            ),
            public_network_access: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccess").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            shard_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("shardCount").unwrap(),
            ),
            source_location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceLocation").unwrap(),
            ),
            source_server_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceServerId").unwrap(),
            ),
            storage_size_in_gb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageSizeInGb").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
