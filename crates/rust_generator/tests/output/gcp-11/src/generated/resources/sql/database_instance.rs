/// Creates a new Google SQL Database Instance. For more information, see the [official documentation](https://cloud.google.com/sql/),
/// or the [JSON API](https://cloud.google.com/sql/docs/admin-api/v1beta4/instances).
///
/// > **NOTE on `gcp.sql.DatabaseInstance`:** - Second-generation instances include a
/// default 'root'@'%' user with no password. This user will be deleted by the provider on
/// instance creation. You should use `gcp.sql.User` to define a custom user with
/// a restricted host and strong password.
///
/// > **Note**: On newer versions of the provider, you must explicitly set `deletion_protection=false`
/// (and run `pulumi update` to write the field to state) in order to destroy an instance.
/// It is recommended to not set this field (or set it to true) until you're ready to destroy the instance and its databases.
///
/// ## Example Usage
///
/// ### SQL Second Generation Instance
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let main = database_instance::create(
///         "main",
///         DatabaseInstanceArgs::builder()
///             .database_version("POSTGRES_15")
///             .name("main-instance")
///             .region("us-central1")
///             .settings(
///                 DatabaseInstanceSettings::builder().tier("db-f1-micro").build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Private IP Instance
/// > **NOTE:** For private IP instance setup, note that the `gcp.sql.DatabaseInstance` does not actually interpolate values from `gcp.servicenetworking.Connection`. You must explicitly add a `depends_on`reference as shown below.
///
/// ```yaml
/// resources:
///   privateNetwork:
///     type: gcp:compute:Network
///     name: private_network
///     properties:
///       name: private-network
///   privateIpAddress:
///     type: gcp:compute:GlobalAddress
///     name: private_ip_address
///     properties:
///       name: private-ip-address
///       purpose: VPC_PEERING
///       addressType: INTERNAL
///       prefixLength: 16
///       network: ${privateNetwork.id}
///   privateVpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: private_vpc_connection
///     properties:
///       network: ${privateNetwork.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${privateIpAddress.name}
///   dbNameSuffix:
///     type: random:RandomId
///     name: db_name_suffix
///     properties:
///       byteLength: 4
///   instance:
///     type: gcp:sql:DatabaseInstance
///     properties:
///       name: private-instance-${dbNameSuffix.hex}
///       region: us-central1
///       databaseVersion: MYSQL_5_7
///       settings:
///         tier: db-f1-micro
///         ipConfiguration:
///           ipv4Enabled: false
///           privateNetwork: ${privateNetwork.selfLink}
///           enablePrivatePathForGoogleCloudServices: true
///     options:
///       dependsOn:
///         - ${privateVpcConnection}
/// ```
///
/// ### ENTERPRISE_PLUS Instance with data_cache_config
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let main = database_instance::create(
///         "main",
///         DatabaseInstanceArgs::builder()
///             .database_version("MYSQL_8_0_31")
///             .name("enterprise-plus-main-instance")
///             .settings(
///                 DatabaseInstanceSettings::builder()
///                     .dataCacheConfig(
///                         DatabaseInstanceSettingsDataCacheConfig::builder()
///                             .dataCacheEnabled(true)
///                             .build_struct(),
///                     )
///                     .edition("ENTERPRISE_PLUS")
///                     .tier("db-perf-optimized-N-2")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Cloud SQL Instance with PSC connectivity
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let main = database_instance::create(
///         "main",
///         DatabaseInstanceArgs::builder()
///             .database_version("MYSQL_8_0")
///             .name("psc-enabled-main-instance")
///             .settings(
///                 DatabaseInstanceSettings::builder()
///                     .availabilityType("REGIONAL")
///                     .backupConfiguration(
///                         DatabaseInstanceSettingsBackupConfiguration::builder()
///                             .binaryLogEnabled(true)
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .ipConfiguration(
///                         DatabaseInstanceSettingsIpConfiguration::builder()
///                             .ipv4Enabled(false)
///                             .pscConfigs(
///                                 vec![
///                                     DatabaseInstanceSettingsIpConfigurationPscConfig::builder()
///                                     .allowedConsumerProjects(vec!["allowed-consumer-project-name",])
///                                     .pscEnabled(true).build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .tier("db-f1-micro")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Cloud SQL Instance with PSC auto connections
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let main = database_instance::create(
///         "main",
///         DatabaseInstanceArgs::builder()
///             .database_version("MYSQL_8_0")
///             .name("psc-enabled-main-instance")
///             .settings(
///                 DatabaseInstanceSettings::builder()
///                     .availabilityType("REGIONAL")
///                     .backupConfiguration(
///                         DatabaseInstanceSettingsBackupConfiguration::builder()
///                             .binaryLogEnabled(true)
///                             .enabled(true)
///                             .build_struct(),
///                     )
///                     .ipConfiguration(
///                         DatabaseInstanceSettingsIpConfiguration::builder()
///                             .ipv4Enabled(false)
///                             .pscConfigs(
///                                 vec![
///                                     DatabaseInstanceSettingsIpConfigurationPscConfig::builder()
///                                     .allowedConsumerProjects(vec!["allowed-consumer-project-name",])
///                                     .pscAutoConnections(vec![DatabaseInstanceSettingsIpConfigurationPscConfigPscAutoConnection::builder()
///                                     .consumerNetwork("network-name")
///                                     .consumerServiceProjectId("project-id").build_struct(),])
///                                     .pscEnabled(true).build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .tier("db-f1-micro")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Switchover (SQL Server Only)
///
/// Users can perform a switchover on any direct `cascadable` replica by following the steps below.
///
///   ~>**WARNING:** Failure to follow these steps can lead to data loss (You will be warned during plan stage). To prevent data loss during a switchover, please verify your plan with the checklist below.
///
/// For a more in-depth walkthrough with example code, see the Switchover Guide
///
/// ### Steps to Invoke Switchover
///
/// Create a `cascadable` replica in a different region from the primary (`cascadable_replica` is set to true in `replica_configuration`)
///
/// #### Invoking switchover in the replica resource:
/// 1. Change instance_type from `READ_REPLICA_INSTANCE` to `CLOUD_SQL_INSTANCE`
/// 2. Remove `master_instance_name`
/// 3. Remove `replica_configuration`
/// 4. Add current primary's name to the replica's `replica_names` list
///
/// #### Updating the primary resource:
/// 1. Change `instance_type` from `CLOUD_SQL_INSTANCE` to `READ_REPLICA_INSTANCE`
/// 2. Set `master_instance_name` to the original replica (which will be primary after switchover)
/// 3. Set `replica_configuration` and set `cascadable_replica` to `true`
/// 4. Remove original replica from `replica_names`
///
///     > **NOTE**: Do **not** delete the replica_names field, even if it has no replicas remaining. Set replica_names = [ ] to indicate it having no replicas.
///
/// #### Plan and verify that:
/// - `pulumi preview` outputs **"0 to add, 0 to destroy"**
/// - `pulumi preview` does not say **"must be replaced"** for any resource
/// - Every resource **"will be updated in-place"**
/// - Only the 2 instances involved in switchover have planned changes
/// - (Recommended) Use `deletion_protection` on instances as a safety measure
///
/// ## Import
///
/// Database instances can be imported using one of any of these accepted formats:
///
/// * `projects/{{project}}/instances/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Database instances can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:sql/databaseInstance:DatabaseInstance default projects/{{project}}/instances/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:sql/databaseInstance:DatabaseInstance default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:sql/databaseInstance:DatabaseInstance default {{name}}
/// ```
///
/// config and set on the server.
///
/// When importing, double-check that your config has all the fields set that you expect- just seeing
///
/// no diff isn't sufficient to know that your config could reproduce the imported resource.
///
pub mod database_instance {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatabaseInstanceArgs {
        /// The context needed to create this instance as a clone of another instance. When this field is set during
        /// resource creation, this provider will attempt to clone another instance as indicated in the context. The
        /// configuration is detailed below.
        #[builder(into, default)]
        pub clone: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::sql::DatabaseInstanceClone>,
        >,
        /// The MySQL, PostgreSQL or
        /// SQL Server version to use. Supported values include `MYSQL_5_6`,
        /// `MYSQL_5_7`, `MYSQL_8_0`, `POSTGRES_9_6`,`POSTGRES_10`, `POSTGRES_11`,
        /// `POSTGRES_12`, `POSTGRES_13`, `POSTGRES_14`, `POSTGRES_15`, `POSTGRES_16`, `POSTGRES_17`,
        /// `SQLSERVER_2017_STANDARD`, `SQLSERVER_2017_ENTERPRISE`, `SQLSERVER_2017_EXPRESS`, `SQLSERVER_2017_WEB`.
        /// `SQLSERVER_2019_STANDARD`, `SQLSERVER_2019_ENTERPRISE`, `SQLSERVER_2019_EXPRESS`,
        /// `SQLSERVER_2019_WEB`.
        /// [Database Version Policies](https://cloud.google.com/sql/docs/db-versions)
        /// includes an up-to-date reference of supported versions.
        #[builder(into)]
        pub database_version: pulumi_wasm_rust::InputOrOutput<String>,
        /// Whether or not to allow the provider to destroy the instance. Unless this field is set to false
        /// in state, a `destroy` or `update` command that deletes the instance will fail. Defaults to `true`.
        #[builder(into, default)]
        pub deletion_protection: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// The full path to the encryption key used for the CMEK disk encryption.  Setting
        /// up disk encryption currently requires manual steps outside of this provider.
        /// The provided key must be in the same region as the SQL instance.  In order
        /// to use this feature, a special kind of service account must be created and
        /// granted permission on this key.  This step can currently only be done
        /// manually, please see [this step](https://cloud.google.com/sql/docs/mysql/configure-cmek#service-account).
        /// That service account needs the `Cloud KMS > Cloud KMS CryptoKey Encrypter/Decrypter` role on your
        /// key - please see [this step](https://cloud.google.com/sql/docs/mysql/configure-cmek#grantkey).
        #[builder(into, default)]
        pub encryption_key_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The type of the instance. The supported values are `SQL_INSTANCE_TYPE_UNSPECIFIED`, `CLOUD_SQL_INSTANCE`, `ON_PREMISES_INSTANCE` and `READ_REPLICA_INSTANCE`.
        #[builder(into, default)]
        pub instance_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The current software version on the instance. This attribute can not be set during creation. Refer to `available_maintenance_versions` attribute to see what `maintenance_version` are available for upgrade. When this attribute gets updated, it will cause an instance restart. Setting a `maintenance_version` value that is older than the current one on the instance will be ignored.
        #[builder(into, default)]
        pub maintenance_version: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the existing instance that will
        /// act as the master in the replication setup. Note, this requires the master to
        /// have `binary_log_enabled` set, as well as existing backups.
        #[builder(into, default)]
        pub master_instance_name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the instance. If the name is left
        /// blank, the provider will randomly generate one when the instance is first
        /// created. This is done because after a name is used, it cannot be reused for
        /// up to [one week](https://cloud.google.com/sql/docs/delete-instance).
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The region the instance will sit in. If a region is not provided in the resource definition,
        /// the provider region will be used instead.
        ///
        /// - - -
        #[builder(into, default)]
        pub region: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The configuration for replication. The
        /// configuration is detailed below.
        #[builder(into, default)]
        pub replica_configuration: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::sql::DatabaseInstanceReplicaConfiguration>,
        >,
        /// List of replica names. Can be updated.
        #[builder(into, default)]
        pub replica_names: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// The context needed to restore the database to a backup run. This field will
        /// cause the provider to trigger the database to restore from the backup run indicated. The configuration is detailed below.
        /// **NOTE:** Restoring from a backup is an imperative action and not recommended via this provider. Adding or modifying this
        /// block during resource creation/update will trigger the restore action after the resource is created/updated.
        #[builder(into, default)]
        pub restore_backup_context: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::sql::DatabaseInstanceRestoreBackupContext>,
        >,
        /// Initial root password. Can be updated. Required for MS SQL Server.
        #[builder(into, default)]
        pub root_password: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The settings to use for the database. The
        /// configuration is detailed below. Required if `clone` is not set.
        #[builder(into, default)]
        pub settings: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::sql::DatabaseInstanceSettings>,
        >,
    }
    #[allow(dead_code)]
    pub struct DatabaseInstanceResult {
        /// The list of all maintenance versions applicable on the instance.
        pub available_maintenance_versions: pulumi_wasm_rust::Output<Vec<String>>,
        /// The context needed to create this instance as a clone of another instance. When this field is set during
        /// resource creation, this provider will attempt to clone another instance as indicated in the context. The
        /// configuration is detailed below.
        pub clone: pulumi_wasm_rust::Output<
            Option<super::super::types::sql::DatabaseInstanceClone>,
        >,
        /// The connection name of the instance to be used in
        /// connection strings. For example, when connecting with [Cloud SQL Proxy](https://cloud.google.com/sql/docs/mysql/connect-admin-proxy).
        pub connection_name: pulumi_wasm_rust::Output<String>,
        /// The MySQL, PostgreSQL or
        /// SQL Server version to use. Supported values include `MYSQL_5_6`,
        /// `MYSQL_5_7`, `MYSQL_8_0`, `POSTGRES_9_6`,`POSTGRES_10`, `POSTGRES_11`,
        /// `POSTGRES_12`, `POSTGRES_13`, `POSTGRES_14`, `POSTGRES_15`, `POSTGRES_16`, `POSTGRES_17`,
        /// `SQLSERVER_2017_STANDARD`, `SQLSERVER_2017_ENTERPRISE`, `SQLSERVER_2017_EXPRESS`, `SQLSERVER_2017_WEB`.
        /// `SQLSERVER_2019_STANDARD`, `SQLSERVER_2019_ENTERPRISE`, `SQLSERVER_2019_EXPRESS`,
        /// `SQLSERVER_2019_WEB`.
        /// [Database Version Policies](https://cloud.google.com/sql/docs/db-versions)
        /// includes an up-to-date reference of supported versions.
        pub database_version: pulumi_wasm_rust::Output<String>,
        /// Whether or not to allow the provider to destroy the instance. Unless this field is set to false
        /// in state, a `destroy` or `update` command that deletes the instance will fail. Defaults to `true`.
        pub deletion_protection: pulumi_wasm_rust::Output<Option<bool>>,
        /// The DNS name of the instance. See [Connect to an instance using Private Service Connect](https://cloud.google.com/sql/docs/mysql/configure-private-service-connect#view-summary-information-cloud-sql-instances-psc-enabled) for more details.
        pub dns_name: pulumi_wasm_rust::Output<String>,
        /// The full path to the encryption key used for the CMEK disk encryption.  Setting
        /// up disk encryption currently requires manual steps outside of this provider.
        /// The provided key must be in the same region as the SQL instance.  In order
        /// to use this feature, a special kind of service account must be created and
        /// granted permission on this key.  This step can currently only be done
        /// manually, please see [this step](https://cloud.google.com/sql/docs/mysql/configure-cmek#service-account).
        /// That service account needs the `Cloud KMS > Cloud KMS CryptoKey Encrypter/Decrypter` role on your
        /// key - please see [this step](https://cloud.google.com/sql/docs/mysql/configure-cmek#grantkey).
        pub encryption_key_name: pulumi_wasm_rust::Output<String>,
        /// The first IPv4 address of any type assigned.
        pub first_ip_address: pulumi_wasm_rust::Output<String>,
        /// The type of the instance. The supported values are `SQL_INSTANCE_TYPE_UNSPECIFIED`, `CLOUD_SQL_INSTANCE`, `ON_PREMISES_INSTANCE` and `READ_REPLICA_INSTANCE`.
        pub instance_type: pulumi_wasm_rust::Output<String>,
        pub ip_addresses: pulumi_wasm_rust::Output<
            Vec<super::super::types::sql::DatabaseInstanceIpAddress>,
        >,
        /// The current software version on the instance. This attribute can not be set during creation. Refer to `available_maintenance_versions` attribute to see what `maintenance_version` are available for upgrade. When this attribute gets updated, it will cause an instance restart. Setting a `maintenance_version` value that is older than the current one on the instance will be ignored.
        pub maintenance_version: pulumi_wasm_rust::Output<String>,
        /// The name of the existing instance that will
        /// act as the master in the replication setup. Note, this requires the master to
        /// have `binary_log_enabled` set, as well as existing backups.
        pub master_instance_name: pulumi_wasm_rust::Output<String>,
        /// The name of the instance. If the name is left
        /// blank, the provider will randomly generate one when the instance is first
        /// created. This is done because after a name is used, it cannot be reused for
        /// up to [one week](https://cloud.google.com/sql/docs/delete-instance).
        pub name: pulumi_wasm_rust::Output<String>,
        /// The first private (`PRIVATE`) IPv4 address assigned.
        pub private_ip_address: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// the URI that points to the service attachment of the instance.
        pub psc_service_attachment_link: pulumi_wasm_rust::Output<String>,
        /// The first public (`PRIMARY`) IPv4 address assigned.
        pub public_ip_address: pulumi_wasm_rust::Output<String>,
        /// The region the instance will sit in. If a region is not provided in the resource definition,
        /// the provider region will be used instead.
        ///
        /// - - -
        pub region: pulumi_wasm_rust::Output<String>,
        /// The configuration for replication. The
        /// configuration is detailed below.
        pub replica_configuration: pulumi_wasm_rust::Output<
            super::super::types::sql::DatabaseInstanceReplicaConfiguration,
        >,
        /// List of replica names. Can be updated.
        pub replica_names: pulumi_wasm_rust::Output<Vec<String>>,
        /// The context needed to restore the database to a backup run. This field will
        /// cause the provider to trigger the database to restore from the backup run indicated. The configuration is detailed below.
        /// **NOTE:** Restoring from a backup is an imperative action and not recommended via this provider. Adding or modifying this
        /// block during resource creation/update will trigger the restore action after the resource is created/updated.
        pub restore_backup_context: pulumi_wasm_rust::Output<
            Option<super::super::types::sql::DatabaseInstanceRestoreBackupContext>,
        >,
        /// Initial root password. Can be updated. Required for MS SQL Server.
        pub root_password: pulumi_wasm_rust::Output<Option<String>>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
        pub server_ca_certs: pulumi_wasm_rust::Output<
            Vec<super::super::types::sql::DatabaseInstanceServerCaCert>,
        >,
        /// The service account email address assigned to the
        /// instance.
        pub service_account_email_address: pulumi_wasm_rust::Output<String>,
        /// The settings to use for the database. The
        /// configuration is detailed below. Required if `clone` is not set.
        pub settings: pulumi_wasm_rust::Output<
            super::super::types::sql::DatabaseInstanceSettings,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DatabaseInstanceArgs,
    ) -> DatabaseInstanceResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let clone_binding = args.clone.get_output(context).get_inner();
        let database_version_binding = args
            .database_version
            .get_output(context)
            .get_inner();
        let deletion_protection_binding = args
            .deletion_protection
            .get_output(context)
            .get_inner();
        let encryption_key_name_binding = args
            .encryption_key_name
            .get_output(context)
            .get_inner();
        let instance_type_binding = args.instance_type.get_output(context).get_inner();
        let maintenance_version_binding = args
            .maintenance_version
            .get_output(context)
            .get_inner();
        let master_instance_name_binding = args
            .master_instance_name
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let replica_configuration_binding = args
            .replica_configuration
            .get_output(context)
            .get_inner();
        let replica_names_binding = args.replica_names.get_output(context).get_inner();
        let restore_backup_context_binding = args
            .restore_backup_context
            .get_output(context)
            .get_inner();
        let root_password_binding = args.root_password.get_output(context).get_inner();
        let settings_binding = args.settings.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:sql/databaseInstance:DatabaseInstance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clone".into(),
                    value: &clone_binding,
                },
                register_interface::ObjectField {
                    name: "databaseVersion".into(),
                    value: &database_version_binding,
                },
                register_interface::ObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionKeyName".into(),
                    value: &encryption_key_name_binding,
                },
                register_interface::ObjectField {
                    name: "instanceType".into(),
                    value: &instance_type_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceVersion".into(),
                    value: &maintenance_version_binding,
                },
                register_interface::ObjectField {
                    name: "masterInstanceName".into(),
                    value: &master_instance_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "region".into(),
                    value: &region_binding,
                },
                register_interface::ObjectField {
                    name: "replicaConfiguration".into(),
                    value: &replica_configuration_binding,
                },
                register_interface::ObjectField {
                    name: "replicaNames".into(),
                    value: &replica_names_binding,
                },
                register_interface::ObjectField {
                    name: "restoreBackupContext".into(),
                    value: &restore_backup_context_binding,
                },
                register_interface::ObjectField {
                    name: "rootPassword".into(),
                    value: &root_password_binding,
                },
                register_interface::ObjectField {
                    name: "settings".into(),
                    value: &settings_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DatabaseInstanceResult {
            available_maintenance_versions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("availableMaintenanceVersions"),
            ),
            clone: pulumi_wasm_rust::__private::into_domain(o.extract_field("clone")),
            connection_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("connectionName"),
            ),
            database_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("databaseVersion"),
            ),
            deletion_protection: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deletionProtection"),
            ),
            dns_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("dnsName"),
            ),
            encryption_key_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("encryptionKeyName"),
            ),
            first_ip_address: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("firstIpAddress"),
            ),
            instance_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instanceType"),
            ),
            ip_addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ipAddresses"),
            ),
            maintenance_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("maintenanceVersion"),
            ),
            master_instance_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("masterInstanceName"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            private_ip_address: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("privateIpAddress"),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            psc_service_attachment_link: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("pscServiceAttachmentLink"),
            ),
            public_ip_address: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("publicIpAddress"),
            ),
            region: pulumi_wasm_rust::__private::into_domain(o.extract_field("region")),
            replica_configuration: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("replicaConfiguration"),
            ),
            replica_names: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("replicaNames"),
            ),
            restore_backup_context: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("restoreBackupContext"),
            ),
            root_password: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("rootPassword"),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("selfLink"),
            ),
            server_ca_certs: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serverCaCerts"),
            ),
            service_account_email_address: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("serviceAccountEmailAddress"),
            ),
            settings: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("settings"),
            ),
        }
    }
}
