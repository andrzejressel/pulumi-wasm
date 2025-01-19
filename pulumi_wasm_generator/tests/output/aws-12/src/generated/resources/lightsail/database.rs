/// Provides a Lightsail Database. Amazon Lightsail is a service to provide easy virtual private servers
/// with custom software already setup. See [What is Amazon Lightsail?](https://lightsail.aws.amazon.com/ls/docs/getting-started/article/what-is-amazon-lightsail)
/// for more information.
///
/// > **Note:** Lightsail is currently only supported in a limited number of AWS Regions, please see ["Regions and Availability Zones"](https://aws.amazon.com/about-aws/global-infrastructure/regional-product-services/) for more details
///
/// ## Example Usage
///
/// ### Basic mysql blueprint
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = database::create(
///         "test",
///         DatabaseArgs::builder()
///             .availability_zone("us-east-1a")
///             .blueprint_id("mysql_8_0")
///             .bundle_id("micro_1_0")
///             .master_database_name("testdatabasename")
///             .master_password("testdatabasepassword")
///             .master_username("test")
///             .relational_database_name("test")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Basic postrgres blueprint
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = database::create(
///         "test",
///         DatabaseArgs::builder()
///             .availability_zone("us-east-1a")
///             .blueprint_id("postgres_12")
///             .bundle_id("micro_1_0")
///             .master_database_name("testdatabasename")
///             .master_password("testdatabasepassword")
///             .master_username("test")
///             .relational_database_name("test")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Custom backup and maintenance windows
///
/// Below is an example that sets a custom backup and maintenance window. Times are specified in UTC. This example will allow daily backups to take place between 16:00 and 16:30 each day. This example also requires any maintiance tasks (anything that would cause an outage, including changing some attributes) to take place on Tuesdays between 17:00 and 17:30. An action taken against this database that would cause an outage will wait until this time window to make the requested changes.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = database::create(
///         "test",
///         DatabaseArgs::builder()
///             .availability_zone("us-east-1a")
///             .blueprint_id("postgres_12")
///             .bundle_id("micro_1_0")
///             .master_database_name("testdatabasename")
///             .master_password("testdatabasepassword")
///             .master_username("test")
///             .preferred_backup_window("16:00-16:30")
///             .preferred_maintenance_window("Tue:17:00-Tue:17:30")
///             .relational_database_name("test")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Final Snapshots
///
/// To enable creating a final snapshot of your database on deletion, use the `final_snapshot_name` argument to provide a name to be used for the snapshot.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = database::create(
///         "test",
///         DatabaseArgs::builder()
///             .availability_zone("us-east-1a")
///             .blueprint_id("postgres_12")
///             .bundle_id("micro_1_0")
///             .final_snapshot_name("MyFinalSnapshot")
///             .master_database_name("testdatabasename")
///             .master_password("testdatabasepassword")
///             .master_username("test")
///             .preferred_backup_window("16:00-16:30")
///             .preferred_maintenance_window("Tue:17:00-Tue:17:30")
///             .relational_database_name("test")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Apply Immediately
///
/// To enable applying changes immediately instead of waiting for a maintiance window, use the `apply_immediately` argument.
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = database::create(
///         "test",
///         DatabaseArgs::builder()
///             .apply_immediately(true)
///             .availability_zone("us-east-1a")
///             .blueprint_id("postgres_12")
///             .bundle_id("micro_1_0")
///             .master_database_name("testdatabasename")
///             .master_password("testdatabasepassword")
///             .master_username("test")
///             .relational_database_name("test")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Blueprint Ids
///
/// A list of all available Lightsail Blueprints for Relational Databases the [aws lightsail get-relational-database-blueprints](https://docs.aws.amazon.com/cli/latest/reference/lightsail/get-relational-database-blueprints.html) aws cli command.
///
/// ### Examples
///
/// - `mysql_8_0`
/// - `postgres_12`
///
/// ### Prefix
///
/// A Blueprint ID starts with a prefix of the engine type.
///
/// ### Suffix
///
/// A Blueprint ID has a sufix of the engine version.
///
/// ## Bundles
///
/// A list of all available Lightsail Bundles for Relational Databases the [aws lightsail get-relational-database-bundles](https://docs.aws.amazon.com/cli/latest/reference/lightsail/get-relational-database-bundles.html) aws cli command.
///
/// ### Examples
///
/// - `small_1_0`
/// - `small_ha_1_0`
/// - `large_1_0`
/// - `large_ha_1_0`
///
/// ### Prefix
///
/// A Bundle ID starts with one of the below size prefixes:
///
/// - `micro_`
/// - `small_`
/// - `medium_`
/// - `large_`
///
/// ### Infixes (Optional for HA Database)
///
/// A Bundle Id can have the following infix added in order to use the HA option of the selected bundle.
///
/// - `ha_`
///
/// ### Suffix
///
/// A Bundle ID ends with one of the following suffix: `1_0`
///
/// ## Import
///
/// Using `pulumi import`, import Lightsail Databases using their name. For example:
///
/// ```sh
/// $ pulumi import aws:lightsail/database:Database foo 'bar'
/// ```
pub mod database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatabaseArgs {
        /// When true , applies changes immediately. When false , applies changes during the preferred maintenance window. Some changes may cause an outage.
        #[builder(into, default)]
        pub apply_immediately: pulumi_wasm_rust::Output<Option<bool>>,
        /// The Availability Zone in which to create your new database. Use the us-east-2a case-sensitive format.
        #[builder(into, default)]
        pub availability_zone: pulumi_wasm_rust::Output<Option<String>>,
        /// When true, enables automated backup retention for your database. When false, disables automated backup retention for your database. Disabling backup retention deletes all automated database backups. Before disabling this, you may want to create a snapshot of your database.
        #[builder(into, default)]
        pub backup_retention_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The blueprint ID for your new database. A blueprint describes the major engine version of a database. You can get a list of database blueprints IDs by using the AWS CLI command: `aws lightsail get-relational-database-blueprints`
        #[builder(into)]
        pub blueprint_id: pulumi_wasm_rust::Output<String>,
        /// The bundle ID for your new database. A bundle describes the performance specifications for your database (see list below). You can get a list of database bundle IDs by using the AWS CLI command: `aws lightsail get-relational-database-bundles`.
        #[builder(into)]
        pub bundle_id: pulumi_wasm_rust::Output<String>,
        /// The name of the database snapshot created if skip final snapshot is false, which is the default value for that parameter.
        #[builder(into, default)]
        pub final_snapshot_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the master database created when the Lightsail database resource is created.
        #[builder(into)]
        pub master_database_name: pulumi_wasm_rust::Output<String>,
        /// The password for the master user of your new database. The password can include any printable ASCII character except "/", """, or "@".
        #[builder(into)]
        pub master_password: pulumi_wasm_rust::Output<String>,
        /// The master user name for your new database.
        #[builder(into)]
        pub master_username: pulumi_wasm_rust::Output<String>,
        /// The daily time range during which automated backups are created for your new database if automated backups are enabled. Must be in the hh24:mi-hh24:mi format. Example: `16:00-16:30`. Specified in Coordinated Universal Time (UTC).
        #[builder(into, default)]
        pub preferred_backup_window: pulumi_wasm_rust::Output<Option<String>>,
        /// The weekly time range during which system maintenance can occur on your new database. Must be in the ddd:hh24:mi-ddd:hh24:mi format. Specified in Coordinated Universal Time (UTC). Example: `Tue:17:00-Tue:17:30`
        #[builder(into, default)]
        pub preferred_maintenance_window: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies the accessibility options for your new database. A value of true specifies a database that is available to resources outside of your Lightsail account. A value of false specifies a database that is available only to your Lightsail resources in the same region as your database.
        #[builder(into, default)]
        pub publicly_accessible: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name to use for your new Lightsail database resource. Names be unique within each AWS Region in your Lightsail account.
        #[builder(into)]
        pub relational_database_name: pulumi_wasm_rust::Output<String>,
        /// Determines whether a final database snapshot is created before your database is deleted. If true is specified, no database snapshot is created. If false is specified, a database snapshot is created before your database is deleted. You must specify the final relational database snapshot name parameter if the skip final snapshot parameter is false.
        #[builder(into, default)]
        pub skip_final_snapshot: pulumi_wasm_rust::Output<Option<bool>>,
        /// A map of tags to assign to the resource. To create a key-only tag, use an empty string as the value.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DatabaseResult {
        /// When true , applies changes immediately. When false , applies changes during the preferred maintenance window. Some changes may cause an outage.
        pub apply_immediately: pulumi_wasm_rust::Output<bool>,
        /// The ARN of the Lightsail instance (matches `id`).
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The Availability Zone in which to create your new database. Use the us-east-2a case-sensitive format.
        pub availability_zone: pulumi_wasm_rust::Output<String>,
        /// When true, enables automated backup retention for your database. When false, disables automated backup retention for your database. Disabling backup retention deletes all automated database backups. Before disabling this, you may want to create a snapshot of your database.
        pub backup_retention_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// The blueprint ID for your new database. A blueprint describes the major engine version of a database. You can get a list of database blueprints IDs by using the AWS CLI command: `aws lightsail get-relational-database-blueprints`
        pub blueprint_id: pulumi_wasm_rust::Output<String>,
        /// The bundle ID for your new database. A bundle describes the performance specifications for your database (see list below). You can get a list of database bundle IDs by using the AWS CLI command: `aws lightsail get-relational-database-bundles`.
        pub bundle_id: pulumi_wasm_rust::Output<String>,
        /// The certificate associated with the database.
        pub ca_certificate_identifier: pulumi_wasm_rust::Output<String>,
        /// The number of vCPUs for the database.
        pub cpu_count: pulumi_wasm_rust::Output<i32>,
        /// The timestamp when the instance was created.
        pub created_at: pulumi_wasm_rust::Output<String>,
        /// The size of the disk for the database.
        pub disk_size: pulumi_wasm_rust::Output<f64>,
        /// The database software (for example, MySQL).
        pub engine: pulumi_wasm_rust::Output<String>,
        /// The database engine version (for example, 5.7.23).
        pub engine_version: pulumi_wasm_rust::Output<String>,
        /// The name of the database snapshot created if skip final snapshot is false, which is the default value for that parameter.
        pub final_snapshot_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the master database created when the Lightsail database resource is created.
        pub master_database_name: pulumi_wasm_rust::Output<String>,
        /// The master endpoint fqdn for the database.
        pub master_endpoint_address: pulumi_wasm_rust::Output<String>,
        /// The master endpoint network port for the database.
        pub master_endpoint_port: pulumi_wasm_rust::Output<i32>,
        /// The password for the master user of your new database. The password can include any printable ASCII character except "/", """, or "@".
        pub master_password: pulumi_wasm_rust::Output<String>,
        /// The master user name for your new database.
        pub master_username: pulumi_wasm_rust::Output<String>,
        /// The daily time range during which automated backups are created for your new database if automated backups are enabled. Must be in the hh24:mi-hh24:mi format. Example: `16:00-16:30`. Specified in Coordinated Universal Time (UTC).
        pub preferred_backup_window: pulumi_wasm_rust::Output<String>,
        /// The weekly time range during which system maintenance can occur on your new database. Must be in the ddd:hh24:mi-ddd:hh24:mi format. Specified in Coordinated Universal Time (UTC). Example: `Tue:17:00-Tue:17:30`
        pub preferred_maintenance_window: pulumi_wasm_rust::Output<String>,
        /// Specifies the accessibility options for your new database. A value of true specifies a database that is available to resources outside of your Lightsail account. A value of false specifies a database that is available only to your Lightsail resources in the same region as your database.
        pub publicly_accessible: pulumi_wasm_rust::Output<Option<bool>>,
        /// The amount of RAM in GB for the database.
        pub ram_size: pulumi_wasm_rust::Output<f64>,
        /// The name to use for your new Lightsail database resource. Names be unique within each AWS Region in your Lightsail account.
        pub relational_database_name: pulumi_wasm_rust::Output<String>,
        /// Describes the secondary Availability Zone of a high availability database. The secondary database is used for failover support of a high availability database.
        pub secondary_availability_zone: pulumi_wasm_rust::Output<String>,
        /// Determines whether a final database snapshot is created before your database is deleted. If true is specified, no database snapshot is created. If false is specified, a database snapshot is created before your database is deleted. You must specify the final relational database snapshot name parameter if the skip final snapshot parameter is false.
        pub skip_final_snapshot: pulumi_wasm_rust::Output<Option<bool>>,
        /// The support code for the database. Include this code in your email to support when you have questions about a database in Lightsail. This code enables our support team to look up your Lightsail information more easily.
        pub support_code: pulumi_wasm_rust::Output<String>,
        /// A map of tags to assign to the resource. To create a key-only tag, use an empty string as the value.
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
    pub fn create(name: &str, args: DatabaseArgs) -> DatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let apply_immediately_binding = args.apply_immediately.get_inner();
        let availability_zone_binding = args.availability_zone.get_inner();
        let backup_retention_enabled_binding = args.backup_retention_enabled.get_inner();
        let blueprint_id_binding = args.blueprint_id.get_inner();
        let bundle_id_binding = args.bundle_id.get_inner();
        let final_snapshot_name_binding = args.final_snapshot_name.get_inner();
        let master_database_name_binding = args.master_database_name.get_inner();
        let master_password_binding = args.master_password.get_inner();
        let master_username_binding = args.master_username.get_inner();
        let preferred_backup_window_binding = args.preferred_backup_window.get_inner();
        let preferred_maintenance_window_binding = args
            .preferred_maintenance_window
            .get_inner();
        let publicly_accessible_binding = args.publicly_accessible.get_inner();
        let relational_database_name_binding = args.relational_database_name.get_inner();
        let skip_final_snapshot_binding = args.skip_final_snapshot.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:lightsail/database:Database".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applyImmediately".into(),
                    value: &apply_immediately_binding,
                },
                register_interface::ObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding,
                },
                register_interface::ObjectField {
                    name: "backupRetentionEnabled".into(),
                    value: &backup_retention_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "blueprintId".into(),
                    value: &blueprint_id_binding,
                },
                register_interface::ObjectField {
                    name: "bundleId".into(),
                    value: &bundle_id_binding,
                },
                register_interface::ObjectField {
                    name: "finalSnapshotName".into(),
                    value: &final_snapshot_name_binding,
                },
                register_interface::ObjectField {
                    name: "masterDatabaseName".into(),
                    value: &master_database_name_binding,
                },
                register_interface::ObjectField {
                    name: "masterPassword".into(),
                    value: &master_password_binding,
                },
                register_interface::ObjectField {
                    name: "masterUsername".into(),
                    value: &master_username_binding,
                },
                register_interface::ObjectField {
                    name: "preferredBackupWindow".into(),
                    value: &preferred_backup_window_binding,
                },
                register_interface::ObjectField {
                    name: "preferredMaintenanceWindow".into(),
                    value: &preferred_maintenance_window_binding,
                },
                register_interface::ObjectField {
                    name: "publiclyAccessible".into(),
                    value: &publicly_accessible_binding,
                },
                register_interface::ObjectField {
                    name: "relationalDatabaseName".into(),
                    value: &relational_database_name_binding,
                },
                register_interface::ObjectField {
                    name: "skipFinalSnapshot".into(),
                    value: &skip_final_snapshot_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "applyImmediately".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "availabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "backupRetentionEnabled".into(),
                },
                register_interface::ResultField {
                    name: "blueprintId".into(),
                },
                register_interface::ResultField {
                    name: "bundleId".into(),
                },
                register_interface::ResultField {
                    name: "caCertificateIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "cpuCount".into(),
                },
                register_interface::ResultField {
                    name: "createdAt".into(),
                },
                register_interface::ResultField {
                    name: "diskSize".into(),
                },
                register_interface::ResultField {
                    name: "engine".into(),
                },
                register_interface::ResultField {
                    name: "engineVersion".into(),
                },
                register_interface::ResultField {
                    name: "finalSnapshotName".into(),
                },
                register_interface::ResultField {
                    name: "masterDatabaseName".into(),
                },
                register_interface::ResultField {
                    name: "masterEndpointAddress".into(),
                },
                register_interface::ResultField {
                    name: "masterEndpointPort".into(),
                },
                register_interface::ResultField {
                    name: "masterPassword".into(),
                },
                register_interface::ResultField {
                    name: "masterUsername".into(),
                },
                register_interface::ResultField {
                    name: "preferredBackupWindow".into(),
                },
                register_interface::ResultField {
                    name: "preferredMaintenanceWindow".into(),
                },
                register_interface::ResultField {
                    name: "publiclyAccessible".into(),
                },
                register_interface::ResultField {
                    name: "ramSize".into(),
                },
                register_interface::ResultField {
                    name: "relationalDatabaseName".into(),
                },
                register_interface::ResultField {
                    name: "secondaryAvailabilityZone".into(),
                },
                register_interface::ResultField {
                    name: "skipFinalSnapshot".into(),
                },
                register_interface::ResultField {
                    name: "supportCode".into(),
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
        DatabaseResult {
            apply_immediately: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("applyImmediately").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("availabilityZone").unwrap(),
            ),
            backup_retention_enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupRetentionEnabled").unwrap(),
            ),
            blueprint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("blueprintId").unwrap(),
            ),
            bundle_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bundleId").unwrap(),
            ),
            ca_certificate_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("caCertificateIdentifier").unwrap(),
            ),
            cpu_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cpuCount").unwrap(),
            ),
            created_at: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdAt").unwrap(),
            ),
            disk_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskSize").unwrap(),
            ),
            engine: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engine").unwrap(),
            ),
            engine_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("engineVersion").unwrap(),
            ),
            final_snapshot_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("finalSnapshotName").unwrap(),
            ),
            master_database_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterDatabaseName").unwrap(),
            ),
            master_endpoint_address: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterEndpointAddress").unwrap(),
            ),
            master_endpoint_port: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterEndpointPort").unwrap(),
            ),
            master_password: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterPassword").unwrap(),
            ),
            master_username: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("masterUsername").unwrap(),
            ),
            preferred_backup_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredBackupWindow").unwrap(),
            ),
            preferred_maintenance_window: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("preferredMaintenanceWindow").unwrap(),
            ),
            publicly_accessible: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("publiclyAccessible").unwrap(),
            ),
            ram_size: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ramSize").unwrap(),
            ),
            relational_database_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("relationalDatabaseName").unwrap(),
            ),
            secondary_availability_zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secondaryAvailabilityZone").unwrap(),
            ),
            skip_final_snapshot: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("skipFinalSnapshot").unwrap(),
            ),
            support_code: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("supportCode").unwrap(),
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
