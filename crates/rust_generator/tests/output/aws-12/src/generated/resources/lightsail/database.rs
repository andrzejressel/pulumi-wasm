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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatabaseArgs {
        /// When true , applies changes immediately. When false , applies changes during the preferred maintenance window. Some changes may cause an outage.
        #[builder(into, default)]
        pub apply_immediately: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The Availability Zone in which to create your new database. Use the us-east-2a case-sensitive format.
        #[builder(into, default)]
        pub availability_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// When true, enables automated backup retention for your database. When false, disables automated backup retention for your database. Disabling backup retention deletes all automated database backups. Before disabling this, you may want to create a snapshot of your database.
        #[builder(into, default)]
        pub backup_retention_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The blueprint ID for your new database. A blueprint describes the major engine version of a database. You can get a list of database blueprints IDs by using the AWS CLI command: `aws lightsail get-relational-database-blueprints`
        #[builder(into)]
        pub blueprint_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The bundle ID for your new database. A bundle describes the performance specifications for your database (see list below). You can get a list of database bundle IDs by using the AWS CLI command: `aws lightsail get-relational-database-bundles`.
        #[builder(into)]
        pub bundle_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the database snapshot created if skip final snapshot is false, which is the default value for that parameter.
        #[builder(into, default)]
        pub final_snapshot_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the master database created when the Lightsail database resource is created.
        #[builder(into)]
        pub master_database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The password for the master user of your new database. The password can include any printable ASCII character except "/", """, or "@".
        #[builder(into)]
        pub master_password: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The master user name for your new database.
        #[builder(into)]
        pub master_username: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The daily time range during which automated backups are created for your new database if automated backups are enabled. Must be in the hh24:mi-hh24:mi format. Example: `16:00-16:30`. Specified in Coordinated Universal Time (UTC).
        #[builder(into, default)]
        pub preferred_backup_window: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The weekly time range during which system maintenance can occur on your new database. Must be in the ddd:hh24:mi-ddd:hh24:mi format. Specified in Coordinated Universal Time (UTC). Example: `Tue:17:00-Tue:17:30`
        #[builder(into, default)]
        pub preferred_maintenance_window: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies the accessibility options for your new database. A value of true specifies a database that is available to resources outside of your Lightsail account. A value of false specifies a database that is available only to your Lightsail resources in the same region as your database.
        #[builder(into, default)]
        pub publicly_accessible: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name to use for your new Lightsail database resource. Names be unique within each AWS Region in your Lightsail account.
        #[builder(into)]
        pub relational_database_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Determines whether a final database snapshot is created before your database is deleted. If true is specified, no database snapshot is created. If false is specified, a database snapshot is created before your database is deleted. You must specify the final relational database snapshot name parameter if the skip final snapshot parameter is false.
        #[builder(into, default)]
        pub skip_final_snapshot: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// A map of tags to assign to the resource. To create a key-only tag, use an empty string as the value.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DatabaseResult {
        /// When true , applies changes immediately. When false , applies changes during the preferred maintenance window. Some changes may cause an outage.
        pub apply_immediately: pulumi_gestalt_rust::Output<bool>,
        /// The ARN of the Lightsail instance (matches `id`).
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The Availability Zone in which to create your new database. Use the us-east-2a case-sensitive format.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// When true, enables automated backup retention for your database. When false, disables automated backup retention for your database. Disabling backup retention deletes all automated database backups. Before disabling this, you may want to create a snapshot of your database.
        pub backup_retention_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The blueprint ID for your new database. A blueprint describes the major engine version of a database. You can get a list of database blueprints IDs by using the AWS CLI command: `aws lightsail get-relational-database-blueprints`
        pub blueprint_id: pulumi_gestalt_rust::Output<String>,
        /// The bundle ID for your new database. A bundle describes the performance specifications for your database (see list below). You can get a list of database bundle IDs by using the AWS CLI command: `aws lightsail get-relational-database-bundles`.
        pub bundle_id: pulumi_gestalt_rust::Output<String>,
        /// The certificate associated with the database.
        pub ca_certificate_identifier: pulumi_gestalt_rust::Output<String>,
        /// The number of vCPUs for the database.
        pub cpu_count: pulumi_gestalt_rust::Output<i32>,
        /// The timestamp when the instance was created.
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// The size of the disk for the database.
        pub disk_size: pulumi_gestalt_rust::Output<f64>,
        /// The database software (for example, MySQL).
        pub engine: pulumi_gestalt_rust::Output<String>,
        /// The database engine version (for example, 5.7.23).
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// The name of the database snapshot created if skip final snapshot is false, which is the default value for that parameter.
        pub final_snapshot_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the master database created when the Lightsail database resource is created.
        pub master_database_name: pulumi_gestalt_rust::Output<String>,
        /// The master endpoint fqdn for the database.
        pub master_endpoint_address: pulumi_gestalt_rust::Output<String>,
        /// The master endpoint network port for the database.
        pub master_endpoint_port: pulumi_gestalt_rust::Output<i32>,
        /// The password for the master user of your new database. The password can include any printable ASCII character except "/", """, or "@".
        pub master_password: pulumi_gestalt_rust::Output<String>,
        /// The master user name for your new database.
        pub master_username: pulumi_gestalt_rust::Output<String>,
        /// The daily time range during which automated backups are created for your new database if automated backups are enabled. Must be in the hh24:mi-hh24:mi format. Example: `16:00-16:30`. Specified in Coordinated Universal Time (UTC).
        pub preferred_backup_window: pulumi_gestalt_rust::Output<String>,
        /// The weekly time range during which system maintenance can occur on your new database. Must be in the ddd:hh24:mi-ddd:hh24:mi format. Specified in Coordinated Universal Time (UTC). Example: `Tue:17:00-Tue:17:30`
        pub preferred_maintenance_window: pulumi_gestalt_rust::Output<String>,
        /// Specifies the accessibility options for your new database. A value of true specifies a database that is available to resources outside of your Lightsail account. A value of false specifies a database that is available only to your Lightsail resources in the same region as your database.
        pub publicly_accessible: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The amount of RAM in GB for the database.
        pub ram_size: pulumi_gestalt_rust::Output<f64>,
        /// The name to use for your new Lightsail database resource. Names be unique within each AWS Region in your Lightsail account.
        pub relational_database_name: pulumi_gestalt_rust::Output<String>,
        /// Describes the secondary Availability Zone of a high availability database. The secondary database is used for failover support of a high availability database.
        pub secondary_availability_zone: pulumi_gestalt_rust::Output<String>,
        /// Determines whether a final database snapshot is created before your database is deleted. If true is specified, no database snapshot is created. If false is specified, a database snapshot is created before your database is deleted. You must specify the final relational database snapshot name parameter if the skip final snapshot parameter is false.
        pub skip_final_snapshot: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The support code for the database. Include this code in your email to support when you have questions about a database in Lightsail. This code enables our support team to look up your Lightsail information more easily.
        pub support_code: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. To create a key-only tag, use an empty string as the value.
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
        args: DatabaseArgs,
    ) -> DatabaseResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let apply_immediately_binding = args.apply_immediately.get_output(context);
        let availability_zone_binding = args.availability_zone.get_output(context);
        let backup_retention_enabled_binding = args
            .backup_retention_enabled
            .get_output(context);
        let blueprint_id_binding = args.blueprint_id.get_output(context);
        let bundle_id_binding = args.bundle_id.get_output(context);
        let final_snapshot_name_binding = args.final_snapshot_name.get_output(context);
        let master_database_name_binding = args.master_database_name.get_output(context);
        let master_password_binding = args.master_password.get_output(context);
        let master_username_binding = args.master_username.get_output(context);
        let preferred_backup_window_binding = args
            .preferred_backup_window
            .get_output(context);
        let preferred_maintenance_window_binding = args
            .preferred_maintenance_window
            .get_output(context);
        let publicly_accessible_binding = args.publicly_accessible.get_output(context);
        let relational_database_name_binding = args
            .relational_database_name
            .get_output(context);
        let skip_final_snapshot_binding = args.skip_final_snapshot.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:lightsail/database:Database".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applyImmediately".into(),
                    value: &apply_immediately_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "backupRetentionEnabled".into(),
                    value: &backup_retention_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blueprintId".into(),
                    value: &blueprint_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bundleId".into(),
                    value: &bundle_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "finalSnapshotName".into(),
                    value: &final_snapshot_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "masterDatabaseName".into(),
                    value: &master_database_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "masterPassword".into(),
                    value: &master_password_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "masterUsername".into(),
                    value: &master_username_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredBackupWindow".into(),
                    value: &preferred_backup_window_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "preferredMaintenanceWindow".into(),
                    value: &preferred_maintenance_window_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "publiclyAccessible".into(),
                    value: &publicly_accessible_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "relationalDatabaseName".into(),
                    value: &relational_database_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipFinalSnapshot".into(),
                    value: &skip_final_snapshot_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DatabaseResult {
            apply_immediately: o.get_field("applyImmediately"),
            arn: o.get_field("arn"),
            availability_zone: o.get_field("availabilityZone"),
            backup_retention_enabled: o.get_field("backupRetentionEnabled"),
            blueprint_id: o.get_field("blueprintId"),
            bundle_id: o.get_field("bundleId"),
            ca_certificate_identifier: o.get_field("caCertificateIdentifier"),
            cpu_count: o.get_field("cpuCount"),
            created_at: o.get_field("createdAt"),
            disk_size: o.get_field("diskSize"),
            engine: o.get_field("engine"),
            engine_version: o.get_field("engineVersion"),
            final_snapshot_name: o.get_field("finalSnapshotName"),
            master_database_name: o.get_field("masterDatabaseName"),
            master_endpoint_address: o.get_field("masterEndpointAddress"),
            master_endpoint_port: o.get_field("masterEndpointPort"),
            master_password: o.get_field("masterPassword"),
            master_username: o.get_field("masterUsername"),
            preferred_backup_window: o.get_field("preferredBackupWindow"),
            preferred_maintenance_window: o.get_field("preferredMaintenanceWindow"),
            publicly_accessible: o.get_field("publiclyAccessible"),
            ram_size: o.get_field("ramSize"),
            relational_database_name: o.get_field("relationalDatabaseName"),
            secondary_availability_zone: o.get_field("secondaryAvailabilityZone"),
            skip_final_snapshot: o.get_field("skipFinalSnapshot"),
            support_code: o.get_field("supportCode"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
