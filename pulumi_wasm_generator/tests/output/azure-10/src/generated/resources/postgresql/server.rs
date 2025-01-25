/// Manages a PostgreSQL Server.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleServer = server::create(
///         "exampleServer",
///         ServerArgs::builder()
///             .administrator_login("psqladmin")
///             .administrator_login_password("H@Sh1CoR3!")
///             .auto_grow_enabled(true)
///             .backup_retention_days(7)
///             .geo_redundant_backup_enabled(true)
///             .location("${example.location}")
///             .name("example-psqlserver")
///             .public_network_access_enabled(false)
///             .resource_group_name("${example.name}")
///             .sku_name("GP_Gen5_4")
///             .ssl_enforcement_enabled(true)
///             .ssl_minimal_tls_version_enforced("TLS1_2")
///             .storage_mb(640000)
///             .version("11")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// PostgreSQL Server's can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:postgresql/server:Server server1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.DBforPostgreSQL/servers/server1
/// ```
///
pub mod server {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServerArgs {
        /// The Administrator login for the PostgreSQL Server. Required when `create_mode` is `Default`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub administrator_login: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Password associated with the `administrator_login` for the PostgreSQL Server. Required when `create_mode` is `Default`.
        #[builder(into, default)]
        pub administrator_login_password: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Enable/Disable auto-growing of the storage. Storage auto-grow prevents your server from running out of storage and becoming read-only. If storage auto grow is enabled, the storage automatically grows without impacting the workload. Defaults to `true`.
        #[builder(into, default)]
        pub auto_grow_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Backup retention days for the server, supported values are between `7` and `35` days.
        #[builder(into, default)]
        pub backup_retention_days: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// The creation mode. Can be used to restore or replicate existing servers. Possible values are `Default`, `Replica`, `GeoRestore`, and `PointInTimeRestore`. Defaults to `Default`.
        #[builder(into, default)]
        pub create_mode: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// For creation modes other than `Default`, the source server ID to use.
        #[builder(into, default)]
        pub creation_source_server_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Turn Geo-redundant server backups on/off. This allows you to choose between locally redundant or geo-redundant backup storage in the General Purpose and Memory Optimized tiers. When the backups are stored in geo-redundant backup storage, they are not only stored within the region in which your server is hosted, but are also replicated to a paired data center. This provides better protection and ability to restore your server in a different region in the event of a disaster. This is not support for the Basic tier. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub geo_redundant_backup_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::postgresql::ServerIdentity>,
        >,
        /// Whether or not infrastructure is encrypted for this server. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This property is currently still in development and not supported by Microsoft. If the `infrastructure_encryption_enabled` attribute is set to `true` the PostgreSQL instance will incur a substantial performance degradation due to a second encryption pass on top of the existing default encryption that is already provided by Azure Storage. It is strongly suggested to leave this value `false` as not doing so can lead to unclear error messages.
        #[builder(into, default)]
        pub infrastructure_encryption_enabled: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the PostgreSQL Server. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Whether or not public network access is allowed for this server. Defaults to `true`.
        #[builder(into, default)]
        pub public_network_access_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The name of the resource group in which to create the PostgreSQL Server. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// When `create_mode` is `PointInTimeRestore` the point in time to restore from `creation_source_server_id`. It should be provided in [RFC3339](https://www.rfc-editor.org/rfc/rfc3339) format, e.g. `2013-11-08T22:00:40Z`.
        #[builder(into, default)]
        pub restore_point_in_time: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies the SKU Name for this PostgreSQL Server. The name of the SKU, follows the `tier` + `family` + `cores` pattern (e.g. `B_Gen4_1`, `GP_Gen5_8`). For more information see the [product documentation](https://docs.microsoft.com/rest/api/postgresql/singleserver/servers/create#sku). Possible values are `B_Gen4_1`, `B_Gen4_2`, `B_Gen5_1`, `B_Gen5_2`, `GP_Gen4_2`, `GP_Gen4_4`, `GP_Gen4_8`, `GP_Gen4_16`, `GP_Gen4_32`, `GP_Gen5_2`, `GP_Gen5_4`, `GP_Gen5_8`, `GP_Gen5_16`, `GP_Gen5_32`, `GP_Gen5_64`, `MO_Gen5_2`, `MO_Gen5_4`, `MO_Gen5_8`, `MO_Gen5_16` and `MO_Gen5_32`.
        ///
        /// > **NOTE:** When replication is set up and `sku_name` is changed to a higher tier or more capacity for the primary, all replicas are scaled up to the same tier/capacity. This is an Azure requirement, for more information see the [replica scaling documentation](https://docs.microsoft.com/azure/postgresql/concepts-read-replicas#scaling)
        #[builder(into)]
        pub sku_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies if SSL should be enforced on connections. Possible values are `true` and `false`.
        ///
        /// > **NOTE:** `ssl_minimal_tls_version_enforced` must be set to `TLSEnforcementDisabled` when `ssl_enforcement_enabled` is set to `false`.
        #[builder(into)]
        pub ssl_enforcement_enabled: pulumi_wasm_rust::InputOrOutput<bool>,
        /// The minimum TLS version to support on the sever. Possible values are `TLSEnforcementDisabled`, `TLS1_0`, `TLS1_1`, and `TLS1_2`. Defaults to `TLS1_2`.
        #[builder(into, default)]
        pub ssl_minimal_tls_version_enforced: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// Max storage allowed for a server. Possible values are between `5120` MB(5GB) and `1048576` MB(1TB) for the Basic SKU and between `5120` MB(5GB) and `16777216` MB(16TB) for General Purpose/Memory Optimized SKUs. For more information see the [product documentation](https://docs.microsoft.com/azure/postgresql/concepts-pricing-tiers#storage).
        #[builder(into, default)]
        pub storage_mb: pulumi_wasm_rust::InputOrOutput<Option<i32>>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Threat detection policy configuration, known in the API as Server Security Alerts Policy. The `threat_detection_policy` block supports fields documented below.
        #[builder(into, default)]
        pub threat_detection_policy: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::postgresql::ServerThreatDetectionPolicy>,
        >,
        /// Specifies the version of PostgreSQL to use. Valid values are `9.5`, `9.6`, `10`, `10.0`, `10.2` and `11`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub version: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServerResult {
        /// The Administrator login for the PostgreSQL Server. Required when `create_mode` is `Default`. Changing this forces a new resource to be created.
        pub administrator_login: pulumi_wasm_rust::Output<String>,
        /// The Password associated with the `administrator_login` for the PostgreSQL Server. Required when `create_mode` is `Default`.
        pub administrator_login_password: pulumi_wasm_rust::Output<Option<String>>,
        /// Enable/Disable auto-growing of the storage. Storage auto-grow prevents your server from running out of storage and becoming read-only. If storage auto grow is enabled, the storage automatically grows without impacting the workload. Defaults to `true`.
        pub auto_grow_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Backup retention days for the server, supported values are between `7` and `35` days.
        pub backup_retention_days: pulumi_wasm_rust::Output<i32>,
        /// The creation mode. Can be used to restore or replicate existing servers. Possible values are `Default`, `Replica`, `GeoRestore`, and `PointInTimeRestore`. Defaults to `Default`.
        pub create_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// For creation modes other than `Default`, the source server ID to use.
        pub creation_source_server_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The FQDN of the PostgreSQL Server.
        pub fqdn: pulumi_wasm_rust::Output<String>,
        /// Turn Geo-redundant server backups on/off. This allows you to choose between locally redundant or geo-redundant backup storage in the General Purpose and Memory Optimized tiers. When the backups are stored in geo-redundant backup storage, they are not only stored within the region in which your server is hosted, but are also replicated to a paired data center. This provides better protection and ability to restore your server in a different region in the event of a disaster. This is not support for the Basic tier. Changing this forces a new resource to be created.
        pub geo_redundant_backup_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            Option<super::super::types::postgresql::ServerIdentity>,
        >,
        /// Whether or not infrastructure is encrypted for this server. Changing this forces a new resource to be created.
        ///
        /// > **NOTE:** This property is currently still in development and not supported by Microsoft. If the `infrastructure_encryption_enabled` attribute is set to `true` the PostgreSQL instance will incur a substantial performance degradation due to a second encryption pass on top of the existing default encryption that is already provided by Azure Storage. It is strongly suggested to leave this value `false` as not doing so can lead to unclear error messages.
        pub infrastructure_encryption_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the PostgreSQL Server. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Whether or not public network access is allowed for this server. Defaults to `true`.
        pub public_network_access_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the resource group in which to create the PostgreSQL Server. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// When `create_mode` is `PointInTimeRestore` the point in time to restore from `creation_source_server_id`. It should be provided in [RFC3339](https://www.rfc-editor.org/rfc/rfc3339) format, e.g. `2013-11-08T22:00:40Z`.
        pub restore_point_in_time: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the SKU Name for this PostgreSQL Server. The name of the SKU, follows the `tier` + `family` + `cores` pattern (e.g. `B_Gen4_1`, `GP_Gen5_8`). For more information see the [product documentation](https://docs.microsoft.com/rest/api/postgresql/singleserver/servers/create#sku). Possible values are `B_Gen4_1`, `B_Gen4_2`, `B_Gen5_1`, `B_Gen5_2`, `GP_Gen4_2`, `GP_Gen4_4`, `GP_Gen4_8`, `GP_Gen4_16`, `GP_Gen4_32`, `GP_Gen5_2`, `GP_Gen5_4`, `GP_Gen5_8`, `GP_Gen5_16`, `GP_Gen5_32`, `GP_Gen5_64`, `MO_Gen5_2`, `MO_Gen5_4`, `MO_Gen5_8`, `MO_Gen5_16` and `MO_Gen5_32`.
        ///
        /// > **NOTE:** When replication is set up and `sku_name` is changed to a higher tier or more capacity for the primary, all replicas are scaled up to the same tier/capacity. This is an Azure requirement, for more information see the [replica scaling documentation](https://docs.microsoft.com/azure/postgresql/concepts-read-replicas#scaling)
        pub sku_name: pulumi_wasm_rust::Output<String>,
        /// Specifies if SSL should be enforced on connections. Possible values are `true` and `false`.
        ///
        /// > **NOTE:** `ssl_minimal_tls_version_enforced` must be set to `TLSEnforcementDisabled` when `ssl_enforcement_enabled` is set to `false`.
        pub ssl_enforcement_enabled: pulumi_wasm_rust::Output<bool>,
        /// The minimum TLS version to support on the sever. Possible values are `TLSEnforcementDisabled`, `TLS1_0`, `TLS1_1`, and `TLS1_2`. Defaults to `TLS1_2`.
        pub ssl_minimal_tls_version_enforced: pulumi_wasm_rust::Output<Option<String>>,
        /// Max storage allowed for a server. Possible values are between `5120` MB(5GB) and `1048576` MB(1TB) for the Basic SKU and between `5120` MB(5GB) and `16777216` MB(16TB) for General Purpose/Memory Optimized SKUs. For more information see the [product documentation](https://docs.microsoft.com/azure/postgresql/concepts-pricing-tiers#storage).
        pub storage_mb: pulumi_wasm_rust::Output<i32>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Threat detection policy configuration, known in the API as Server Security Alerts Policy. The `threat_detection_policy` block supports fields documented below.
        pub threat_detection_policy: pulumi_wasm_rust::Output<
            Option<super::super::types::postgresql::ServerThreatDetectionPolicy>,
        >,
        /// Specifies the version of PostgreSQL to use. Valid values are `9.5`, `9.6`, `10`, `10.0`, `10.2` and `11`. Changing this forces a new resource to be created.
        pub version: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ServerArgs,
    ) -> ServerResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let administrator_login_binding = args
            .administrator_login
            .get_output(context)
            .get_inner();
        let administrator_login_password_binding = args
            .administrator_login_password
            .get_output(context)
            .get_inner();
        let auto_grow_enabled_binding = args
            .auto_grow_enabled
            .get_output(context)
            .get_inner();
        let backup_retention_days_binding = args
            .backup_retention_days
            .get_output(context)
            .get_inner();
        let create_mode_binding = args.create_mode.get_output(context).get_inner();
        let creation_source_server_id_binding = args
            .creation_source_server_id
            .get_output(context)
            .get_inner();
        let geo_redundant_backup_enabled_binding = args
            .geo_redundant_backup_enabled
            .get_output(context)
            .get_inner();
        let identity_binding = args.identity.get_output(context).get_inner();
        let infrastructure_encryption_enabled_binding = args
            .infrastructure_encryption_enabled
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let public_network_access_enabled_binding = args
            .public_network_access_enabled
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let restore_point_in_time_binding = args
            .restore_point_in_time
            .get_output(context)
            .get_inner();
        let sku_name_binding = args.sku_name.get_output(context).get_inner();
        let ssl_enforcement_enabled_binding = args
            .ssl_enforcement_enabled
            .get_output(context)
            .get_inner();
        let ssl_minimal_tls_version_enforced_binding = args
            .ssl_minimal_tls_version_enforced
            .get_output(context)
            .get_inner();
        let storage_mb_binding = args.storage_mb.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let threat_detection_policy_binding = args
            .threat_detection_policy
            .get_output(context)
            .get_inner();
        let version_binding = args.version.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:postgresql/server:Server".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "administratorLogin".into(),
                    value: &administrator_login_binding,
                },
                register_interface::ObjectField {
                    name: "administratorLoginPassword".into(),
                    value: &administrator_login_password_binding,
                },
                register_interface::ObjectField {
                    name: "autoGrowEnabled".into(),
                    value: &auto_grow_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "backupRetentionDays".into(),
                    value: &backup_retention_days_binding,
                },
                register_interface::ObjectField {
                    name: "createMode".into(),
                    value: &create_mode_binding,
                },
                register_interface::ObjectField {
                    name: "creationSourceServerId".into(),
                    value: &creation_source_server_id_binding,
                },
                register_interface::ObjectField {
                    name: "geoRedundantBackupEnabled".into(),
                    value: &geo_redundant_backup_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "infrastructureEncryptionEnabled".into(),
                    value: &infrastructure_encryption_enabled_binding,
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
                    name: "publicNetworkAccessEnabled".into(),
                    value: &public_network_access_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "restorePointInTime".into(),
                    value: &restore_point_in_time_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "sslEnforcementEnabled".into(),
                    value: &ssl_enforcement_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "sslMinimalTlsVersionEnforced".into(),
                    value: &ssl_minimal_tls_version_enforced_binding,
                },
                register_interface::ObjectField {
                    name: "storageMb".into(),
                    value: &storage_mb_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "threatDetectionPolicy".into(),
                    value: &threat_detection_policy_binding,
                },
                register_interface::ObjectField {
                    name: "version".into(),
                    value: &version_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "administratorLogin".into(),
                },
                register_interface::ResultField {
                    name: "administratorLoginPassword".into(),
                },
                register_interface::ResultField {
                    name: "autoGrowEnabled".into(),
                },
                register_interface::ResultField {
                    name: "backupRetentionDays".into(),
                },
                register_interface::ResultField {
                    name: "createMode".into(),
                },
                register_interface::ResultField {
                    name: "creationSourceServerId".into(),
                },
                register_interface::ResultField {
                    name: "fqdn".into(),
                },
                register_interface::ResultField {
                    name: "geoRedundantBackupEnabled".into(),
                },
                register_interface::ResultField {
                    name: "identity".into(),
                },
                register_interface::ResultField {
                    name: "infrastructureEncryptionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "publicNetworkAccessEnabled".into(),
                },
                register_interface::ResultField {
                    name: "resourceGroupName".into(),
                },
                register_interface::ResultField {
                    name: "restorePointInTime".into(),
                },
                register_interface::ResultField {
                    name: "skuName".into(),
                },
                register_interface::ResultField {
                    name: "sslEnforcementEnabled".into(),
                },
                register_interface::ResultField {
                    name: "sslMinimalTlsVersionEnforced".into(),
                },
                register_interface::ResultField {
                    name: "storageMb".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "threatDetectionPolicy".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ServerResult {
            administrator_login: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("administratorLogin").unwrap(),
            ),
            administrator_login_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("administratorLoginPassword").unwrap(),
            ),
            auto_grow_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("autoGrowEnabled").unwrap(),
            ),
            backup_retention_days: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupRetentionDays").unwrap(),
            ),
            create_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createMode").unwrap(),
            ),
            creation_source_server_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationSourceServerId").unwrap(),
            ),
            fqdn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("fqdn").unwrap(),
            ),
            geo_redundant_backup_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("geoRedundantBackupEnabled").unwrap(),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("identity").unwrap(),
            ),
            infrastructure_encryption_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("infrastructureEncryptionEnabled").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            public_network_access_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publicNetworkAccessEnabled").unwrap(),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceGroupName").unwrap(),
            ),
            restore_point_in_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restorePointInTime").unwrap(),
            ),
            sku_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skuName").unwrap(),
            ),
            ssl_enforcement_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sslEnforcementEnabled").unwrap(),
            ),
            ssl_minimal_tls_version_enforced: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sslMinimalTlsVersionEnforced").unwrap(),
            ),
            storage_mb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageMb").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            threat_detection_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("threatDetectionPolicy").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
