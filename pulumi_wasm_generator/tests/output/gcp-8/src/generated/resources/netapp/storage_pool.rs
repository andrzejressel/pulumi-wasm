/// ## Example Usage
///
/// ### Storage Pool Create
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = connection::create(
///         "default",
///         ConnectionArgs::builder()
///             .network("${peeringNetwork.id}")
///             .reserved_peering_ranges(vec!["${privateIpAlloc.name}",])
///             .service("netapp.servicenetworking.goog")
///             .build_struct(),
///     );
///     let peeringNetwork = network::create(
///         "peeringNetwork",
///         NetworkArgs::builder().name("test-network").build_struct(),
///     );
///     let privateIpAlloc = global_address::create(
///         "privateIpAlloc",
///         GlobalAddressArgs::builder()
///             .address_type("INTERNAL")
///             .name("test-address")
///             .network("${peeringNetwork.id}")
///             .prefix_length(16)
///             .purpose("VPC_PEERING")
///             .build_struct(),
///     );
///     let routeUpdates = network_peering_routes_config::create(
///         "routeUpdates",
///         NetworkPeeringRoutesConfigArgs::builder()
///             .export_custom_routes(true)
///             .import_custom_routes(true)
///             .network("${peeringNetwork.name}")
///             .peering("${default.peering}")
///             .build_struct(),
///     );
///     let testPool = storage_pool::create(
///         "testPool",
///         StoragePoolArgs::builder()
///             .capacity_gib("2048")
///             .location("us-central1")
///             .name("test-pool")
///             .network("${peeringNetwork.id}")
///             .service_level("PREMIUM")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// StoragePool can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/storagePools/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, StoragePool can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:netapp/storagePool:StoragePool default projects/{{project}}/locations/{{location}}/storagePools/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:netapp/storagePool:StoragePool default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:netapp/storagePool:StoragePool default {{location}}/{{name}}
/// ```
///
pub mod storage_pool {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StoragePoolArgs {
        /// Specifies the Active Directory policy to be used. Format: `projects/{{project}}/locations/{{location}}/activeDirectories/{{name}}`.
        /// The policy needs to be in the same location as the storage pool.
        #[builder(into, default)]
        pub active_directory: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Optional. True if the storage pool supports Auto Tiering enabled volumes. Default is false.
        /// Auto-tiering can be enabled after storage pool creation but it can't be disabled once enabled.
        #[builder(into, default)]
        pub allow_auto_tiering: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Capacity of the storage pool (in GiB).
        #[builder(into)]
        pub capacity_gib: pulumi_wasm_rust::InputOrOutput<String>,
        /// An optional description of this resource.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the CMEK policy to be used for volume encryption. Format: `projects/{{project}}/locations/{{location}}/kmsConfigs/{{name}}`.
        /// The policy needs to be in the same location as the storage pool.
        #[builder(into, default)]
        pub kms_config: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Labels as key value pairs. Example: `{ "owner": "Bob", "department": "finance", "purpose": "testing" }`.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// When enabled, the volumes uses Active Directory as LDAP name service for UID/GID lookups. Required to enable extended group support for NFSv3,
        /// using security identifiers for NFSv4.1 or principal names for kerberized NFSv4.1.
        #[builder(into, default)]
        pub ldap_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Name of the location. For zonal Flex pools specify a zone name, in all other cases a region name.
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The resource name of the storage pool. Needs to be unique per location/region.
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// VPC network name with format: `projects/{{project}}/global/networks/{{network}}`
        #[builder(into)]
        pub network: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the replica zone for regional Flex pools. `zone` and `replica_zone` values can be swapped to initiate a
        /// [zone switch](https://cloud.google.com/netapp/volumes/docs/configure-and-use/storage-pools/edit-or-delete-storage-pool#switch_active_and_replica_zones).
        #[builder(into, default)]
        pub replica_zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Service level of the storage pool.
        /// Possible values are: `PREMIUM`, `EXTREME`, `STANDARD`, `FLEX`.
        #[builder(into)]
        pub service_level: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the active zone for regional Flex pools. `zone` and `replica_zone` values can be swapped to initiate a
        /// [zone switch](https://cloud.google.com/netapp/volumes/docs/configure-and-use/storage-pools/edit-or-delete-storage-pool#switch_active_and_replica_zones).
        /// If you want to create a zonal Flex pool, specify a zone name for `location` and omit `zone`.
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct StoragePoolResult {
        /// Specifies the Active Directory policy to be used. Format: `projects/{{project}}/locations/{{location}}/activeDirectories/{{name}}`.
        /// The policy needs to be in the same location as the storage pool.
        pub active_directory: pulumi_wasm_rust::Output<Option<String>>,
        /// Optional. True if the storage pool supports Auto Tiering enabled volumes. Default is false.
        /// Auto-tiering can be enabled after storage pool creation but it can't be disabled once enabled.
        pub allow_auto_tiering: pulumi_wasm_rust::Output<Option<bool>>,
        /// Capacity of the storage pool (in GiB).
        pub capacity_gib: pulumi_wasm_rust::Output<String>,
        /// An optional description of this resource.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Reports if volumes in the pool are encrypted using a Google-managed encryption key or CMEK.
        pub encryption_type: pulumi_wasm_rust::Output<String>,
        /// Specifies the CMEK policy to be used for volume encryption. Format: `projects/{{project}}/locations/{{location}}/kmsConfigs/{{name}}`.
        /// The policy needs to be in the same location as the storage pool.
        pub kms_config: pulumi_wasm_rust::Output<Option<String>>,
        /// Labels as key value pairs. Example: `{ "owner": "Bob", "department": "finance", "purpose": "testing" }`.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// When enabled, the volumes uses Active Directory as LDAP name service for UID/GID lookups. Required to enable extended group support for NFSv3,
        /// using security identifiers for NFSv4.1 or principal names for kerberized NFSv4.1.
        pub ldap_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Name of the location. For zonal Flex pools specify a zone name, in all other cases a region name.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The resource name of the storage pool. Needs to be unique per location/region.
        ///
        ///
        /// - - -
        pub name: pulumi_wasm_rust::Output<String>,
        /// VPC network name with format: `projects/{{project}}/global/networks/{{network}}`
        pub network: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Specifies the replica zone for regional Flex pools. `zone` and `replica_zone` values can be swapped to initiate a
        /// [zone switch](https://cloud.google.com/netapp/volumes/docs/configure-and-use/storage-pools/edit-or-delete-storage-pool#switch_active_and_replica_zones).
        pub replica_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// Service level of the storage pool.
        /// Possible values are: `PREMIUM`, `EXTREME`, `STANDARD`, `FLEX`.
        pub service_level: pulumi_wasm_rust::Output<String>,
        /// Size allocated to volumes in the storage pool (in GiB).
        pub volume_capacity_gib: pulumi_wasm_rust::Output<String>,
        /// Number of volume in the storage pool.
        pub volume_count: pulumi_wasm_rust::Output<i32>,
        /// Specifies the active zone for regional Flex pools. `zone` and `replica_zone` values can be swapped to initiate a
        /// [zone switch](https://cloud.google.com/netapp/volumes/docs/configure-and-use/storage-pools/edit-or-delete-storage-pool#switch_active_and_replica_zones).
        /// If you want to create a zonal Flex pool, specify a zone name for `location` and omit `zone`.
        pub zone: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: StoragePoolArgs,
    ) -> StoragePoolResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let active_directory_binding = args
            .active_directory
            .get_output(context)
            .get_inner();
        let allow_auto_tiering_binding = args
            .allow_auto_tiering
            .get_output(context)
            .get_inner();
        let capacity_gib_binding = args.capacity_gib.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let kms_config_binding = args.kms_config.get_output(context).get_inner();
        let labels_binding = args.labels.get_output(context).get_inner();
        let ldap_enabled_binding = args.ldap_enabled.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let network_binding = args.network.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let replica_zone_binding = args.replica_zone.get_output(context).get_inner();
        let service_level_binding = args.service_level.get_output(context).get_inner();
        let zone_binding = args.zone.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:netapp/storagePool:StoragePool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "activeDirectory".into(),
                    value: &active_directory_binding,
                },
                register_interface::ObjectField {
                    name: "allowAutoTiering".into(),
                    value: &allow_auto_tiering_binding,
                },
                register_interface::ObjectField {
                    name: "capacityGib".into(),
                    value: &capacity_gib_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "kmsConfig".into(),
                    value: &kms_config_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "ldapEnabled".into(),
                    value: &ldap_enabled_binding,
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
                    name: "network".into(),
                    value: &network_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "replicaZone".into(),
                    value: &replica_zone_binding,
                },
                register_interface::ObjectField {
                    name: "serviceLevel".into(),
                    value: &service_level_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        StoragePoolResult {
            active_directory: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("activeDirectory"),
            ),
            allow_auto_tiering: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("allowAutoTiering"),
            ),
            capacity_gib: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("capacityGib"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("effectiveLabels"),
            ),
            encryption_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("encryptionType"),
            ),
            kms_config: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kmsConfig"),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(o.extract_field("labels")),
            ldap_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ldapEnabled"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            network: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("network"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pulumiLabels"),
            ),
            replica_zone: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("replicaZone"),
            ),
            service_level: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceLevel"),
            ),
            volume_capacity_gib: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("volumeCapacityGib"),
            ),
            volume_count: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("volumeCount"),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}
