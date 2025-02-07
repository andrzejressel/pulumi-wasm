/// Provides an RDS instance resource.  A DB instance is an isolated database
/// environment in the cloud.  A DB instance can contain multiple user-created
/// databases.
///
/// Changes to a DB instance can occur when you manually change a parameter, such as
/// `allocated_storage`, and are reflected in the next maintenance window. Because
/// of this, this provider may report a difference in its planning phase because a
/// modification has not yet taken place. You can use the `apply_immediately` flag
/// to instruct the service to apply the change immediately (see documentation
/// below).
///
/// When upgrading the major version of an engine, `allow_major_version_upgrade` must be set to `true`.
///
/// > **Note:** using `apply_immediately` can result in a brief downtime as the server reboots.
/// See the AWS Docs on [RDS Instance Maintenance][instance-maintenance] for more information.
///
/// > **Note:** All arguments including the username and password will be stored in the raw state as plain-text.
/// Read more about sensitive data instate.
///
///
///
/// ## RDS Instance Class Types
///
/// Amazon RDS supports instance classes for the following use cases: General-purpose, Memory-optimized, Burstable Performance, and Optimized-reads.
/// For more information please read the AWS RDS documentation about [DB Instance Class Types](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Concepts.DBInstanceClass.html)
///
/// ## Low-Downtime Updates
///
/// By default, RDS applies updates to DB Instances in-place, which can lead to service interruptions.
/// Low-downtime updates minimize service interruptions by performing the updates with an [RDS Blue/Green deployment][blue-green] and switching over the instances when complete.
///
/// Low-downtime updates are only available for DB Instances using MySQL, MariaDB and PostgreSQL,
/// as other engines are not supported by RDS Blue/Green deployments.
/// They cannot be used with DB Instances with replicas.
///
/// Backups must be enabled to use low-downtime updates.
///
/// Enable low-downtime updates by setting `blue_green_update.enabled` to `true`.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = instance::create(
///         "default",
///         InstanceArgs::builder()
///             .allocated_storage(10)
///             .db_name("mydb")
///             .engine("mysql")
///             .engine_version("8.0")
///             .instance_class("db.t3.micro")
///             .parameter_group_name("default.mysql8.0")
///             .password("foobarbaz")
///             .skip_final_snapshot(true)
///             .username("foo")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### RDS Custom for Oracle Usage with Replica
///
/// ```yaml
/// resources:
///   default:
///     type: aws:rds:Instance
///     properties:
///       allocatedStorage: 50
///       autoMinorVersionUpgrade: false # Custom for Oracle does not support minor version upgrades
///       customIamInstanceProfile: AWSRDSCustomInstanceProfile
///       backupRetentionPeriod: 7
///       dbSubnetGroupName: ${dbSubnetGroupName}
///       engine: ${["custom-oracle"].engine}
///       engineVersion: ${["custom-oracle"].engineVersion}
///       identifier: ee-instance-demo
///       instanceClass: ${["custom-oracle"].instanceClass}
///       kmsKeyId: ${byId.arn}
///       licenseModel: ${["custom-oracle"].licenseModel}
///       multiAz: false # Custom for Oracle does not support multi-az
///       password: avoid-plaintext-passwords
///       username: test
///       storageEncrypted: true
///   test-replica:
///     type: aws:rds:Instance
///     properties:
///       replicateSourceDb: ${default.identifier}
///       replicaMode: mounted
///       autoMinorVersionUpgrade: false
///       customIamInstanceProfile: AWSRDSCustomInstanceProfile
///       backupRetentionPeriod: 7
///       identifier: ee-instance-replica
///       instanceClass: ${["custom-oracle"].instanceClass}
///       kmsKeyId: ${byId.arn}
///       multiAz: false # Custom for Oracle does not support multi-az
///       skipFinalSnapshot: true
///       storageEncrypted: true
/// variables:
///   # Lookup the available instance classes for the custom engine for the region being operated in
///   custom-oracle:
///     fn::invoke:
///       function: aws:rds:getOrderableDbInstance
///       arguments:
///         engine: custom-oracle-ee
///         engineVersion: 19.c.ee.002
///         licenseModel: bring-your-own-license
///         storageType: gp3
///         preferredInstanceClasses:
///           - db.r5.xlarge
///           - db.r5.2xlarge
///           - db.r5.4xlarge
///   # The RDS instance resource requires an ARN. Look up the ARN of the KMS key associated with the CEV.
///   byId:
///     fn::invoke:
///       function: aws:kms:getKey
///       arguments:
///         keyId: example-ef278353ceba4a5a97de6784565b9f78
/// ```
///
/// ### RDS Custom for SQL Server
///
/// ```yaml
/// resources:
///   example:
///     type: aws:rds:Instance
///     properties:
///       allocatedStorage: 500
///       autoMinorVersionUpgrade: false # Custom for SQL Server does not support minor version upgrades
///       customIamInstanceProfile: AWSRDSCustomSQLServerInstanceProfile
///       backupRetentionPeriod: 7
///       dbSubnetGroupName: ${dbSubnetGroupName}
///       engine: ${["custom-sqlserver"].engine}
///       engineVersion: ${["custom-sqlserver"].engineVersion}
///       identifier: sql-instance-demo
///       instanceClass: ${["custom-sqlserver"].instanceClass}
///       kmsKeyId: ${byId.arn}
///       multiAz: false # Custom for SQL Server does support multi-az
///       password: avoid-plaintext-passwords
///       storageEncrypted: true
///       username: test
/// variables:
///   # Lookup the available instance classes for the custom engine for the region being operated in
///   custom-sqlserver:
///     fn::invoke:
///       function: aws:rds:getOrderableDbInstance
///       arguments:
///         engine: custom-sqlserver-se
///         engineVersion: 15.00.4249.2.v1
///         storageType: gp3
///         preferredInstanceClasses:
///           - db.r5.xlarge
///           - db.r5.2xlarge
///           - db.r5.4xlarge
///   # The RDS instance resource requires an ARN. Look up the ARN of the KMS key.
///   byId:
///     fn::invoke:
///       function: aws:kms:getKey
///       arguments:
///         keyId: example-ef278353ceba4a5a97de6784565b9f78
/// ```
///
/// ### RDS Db2 Usage
///
/// ```yaml
/// resources:
///   # The RDS Db2 instance resource requires licensing information. Create a new parameter group using the default paramater group as a source, and set license information.
///   exampleParameterGroup:
///     type: aws:rds:ParameterGroup
///     name: example
///     properties:
///       name: db-db2-params
///       family: ${default.parameterGroupFamily}
///       parameters:
///         - applyMethod: immediate
///           name: rds.ibm_customer_id
///           value: 0
///         - applyMethod: immediate
///           name: rds.ibm_site_id
///           value: 0
///   # Create the RDS Db2 instance, use the data sources defined to set attributes
///   exampleInstance:
///     type: aws:rds:Instance
///     name: example
///     properties:
///       allocatedStorage: 100
///       backupRetentionPeriod: 7
///       dbName: test
///       engine: ${example.engine}
///       engineVersion: ${example.engineVersion}
///       identifier: db2-instance-demo
///       instanceClass: ${example.instanceClass}
///       parameterGroupName: ${exampleParameterGroup.name}
///       password: avoid-plaintext-passwords
///       username: test
/// variables:
///   # Lookup the default version for the engine. Db2 Standard Edition is `db2-se`, Db2 Advanced Edition is `db2-ae`.
///   default:
///     fn::invoke:
///       function: aws:rds:getEngineVersion
///       arguments:
///         engine: db2-se
///   # Lookup the available instance classes for the engine in the region being operated in
///   example:
///     fn::invoke:
///       function: aws:rds:getOrderableDbInstance
///       arguments:
///         engine: ${default.engine}
///         engineVersion: ${default.version}
///         licenseModel: bring-your-own-license
///         storageType: gp3
///         preferredInstanceClasses:
///           - db.t3.small
///           - db.r6i.large
///           - db.m6i.large
/// ```
///
/// ### Storage Autoscaling
///
/// To enable Storage Autoscaling with instances that support the feature, define the `max_allocated_storage` argument higher than the `allocated_storage` argument. This provider will automatically hide differences with the `allocated_storage` argument value if autoscaling occurs.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = instance::create(
///         "example",
///         InstanceArgs::builder()
///             .allocated_storage(50)
///             .max_allocated_storage(100)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Managed Master Passwords via Secrets Manager, default KMS Key
///
/// > More information about RDS/Aurora Aurora integrates with Secrets Manager to manage master user passwords for your DB clusters can be found in the [RDS User Guide](https://aws.amazon.com/about-aws/whats-new/2022/12/amazon-rds-integration-aws-secrets-manager/) and [Aurora User Guide](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/rds-secrets-manager.html).
///
/// You can specify the `manage_master_user_password` attribute to enable managing the master password with Secrets Manager. You can also update an existing cluster to use Secrets Manager by specify the `manage_master_user_password` attribute and removing the `password` attribute (removal is required).
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = instance::create(
///         "default",
///         InstanceArgs::builder()
///             .allocated_storage(10)
///             .db_name("mydb")
///             .engine("mysql")
///             .engine_version("8.0")
///             .instance_class("db.t3.micro")
///             .manage_master_user_password(true)
///             .parameter_group_name("default.mysql8.0")
///             .username("foo")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Managed Master Passwords via Secrets Manager, specific KMS Key
///
/// > More information about RDS/Aurora Aurora integrates with Secrets Manager to manage master user passwords for your DB clusters can be found in the [RDS User Guide](https://aws.amazon.com/about-aws/whats-new/2022/12/amazon-rds-integration-aws-secrets-manager/) and [Aurora User Guide](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/rds-secrets-manager.html).
///
/// You can specify the `master_user_secret_kms_key_id` attribute to specify a specific KMS Key.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = instance::create(
///         "default",
///         InstanceArgs::builder()
///             .allocated_storage(10)
///             .db_name("mydb")
///             .engine("mysql")
///             .engine_version("8.0")
///             .instance_class("db.t3.micro")
///             .manage_master_user_password(true)
///             .master_user_secret_kms_key_id("${example.keyId}")
///             .parameter_group_name("default.mysql8.0")
///             .username("foo")
///             .build_struct(),
///     );
///     let example = key::create(
///         "example",
///         KeyArgs::builder().description("Example KMS Key").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import DB Instances using the `identifier`. For example:
///
/// ```sh
/// $ pulumi import aws:rds/instance:Instance default mydb-rds-instance
/// ```
pub mod instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceArgs {
        /// The allocated storage in gibibytes. If `max_allocated_storage` is configured, this argument represents the initial storage allocation and differences from the configuration will be ignored automatically when Storage Autoscaling occurs. If `replicate_source_db` is set, the value is ignored during the creation of the instance.
        #[builder(into, default)]
        pub allocated_storage: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Indicates that major version
        /// upgrades are allowed. Changing this parameter does not result in an outage and
        /// the change is asynchronously applied as soon as possible.
        #[builder(into, default)]
        pub allow_major_version_upgrade: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies whether any database modifications
        /// are applied immediately, or during the next maintenance window. Default is
        /// `false`. See [Amazon RDS Documentation for more
        /// information.](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Overview.DBInstance.Modifying.html)
        #[builder(into, default)]
        pub apply_immediately: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Indicates that minor engine upgrades
        /// will be applied automatically to the DB instance during the maintenance window.
        /// Defaults to true.
        #[builder(into, default)]
        pub auto_minor_version_upgrade: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The AZ for the RDS instance.
        #[builder(into, default)]
        pub availability_zone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The days to retain backups for.
        /// Must be between `0` and `35`.
        /// Default is `0`.
        /// Must be greater than `0` if the database is used as a source for a [Read Replica][instance-replication],
        /// uses low-downtime updates,
        /// or will use [RDS Blue/Green deployments][blue-green].
        #[builder(into, default)]
        pub backup_retention_period: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Specifies where automated backups and manual snapshots are stored. Possible values are `region` (default) and `outposts`. See [Working with Amazon RDS on AWS Outposts](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/rds-on-outposts.html) for more information.
        #[builder(into, default)]
        pub backup_target: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The daily time range (in UTC) during which automated backups are created if they are enabled.
        /// Example: "09:46-10:16". Must not overlap with `maintenance_window`.
        #[builder(into, default)]
        pub backup_window: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Enables low-downtime updates using [RDS Blue/Green deployments][blue-green].
        /// See `blue_green_update` below.
        #[builder(into, default)]
        pub blue_green_update: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rds::InstanceBlueGreenUpdate>,
        >,
        /// The identifier of the CA certificate for the DB instance.
        #[builder(into, default)]
        pub ca_cert_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The character set name to use for DB encoding in Oracle and Microsoft SQL instances (collation).
        /// This can't be changed.
        /// See [Oracle Character Sets Supported in Amazon RDS](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Appendix.OracleCharacterSets.html) or
        /// [Server-Level Collation for Microsoft SQL Server](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Appendix.SQLServer.CommonDBATasks.Collation.html) for more information.
        /// Cannot be set  with `replicate_source_db`, `restore_to_point_in_time`, `s3_import`, or `snapshot_identifier`.
        #[builder(into, default)]
        pub character_set_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Copy all Instance `tags` to snapshots. Default is `false`.
        #[builder(into, default)]
        pub copy_tags_to_snapshot: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The instance profile associated with the underlying Amazon EC2 instance of an RDS Custom DB instance.
        #[builder(into, default)]
        pub custom_iam_instance_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Indicates whether to enable a customer-owned IP address (CoIP) for an RDS on Outposts DB instance. See [CoIP for RDS on Outposts](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/rds-on-outposts.html#rds-on-outposts.coip) for more information.
        ///
        /// > **NOTE:** Removing the `replicate_source_db` attribute from an existing RDS
        /// Replicate database managed by the provider will promote the database to a fully
        /// standalone database.
        #[builder(into, default)]
        pub customer_owned_ip_enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the database to create when the DB instance is created. If this parameter is not specified, no database is created in the DB instance. Note that this does not apply for Oracle or SQL Server engines. See the [AWS documentation](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/rds/create-db-instance.html) for more details on what applies for those engines. If you are providing an Oracle db name, it needs to be in all upper case. Cannot be specified for a replica.
        #[builder(into, default)]
        pub db_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of DB subnet group.
        /// DB instance will be created in the VPC associated with the DB subnet group.
        /// If unspecified, will be created in the `default` Subnet Group.
        /// When working with read replicas created in the same region, defaults to the Subnet Group Name of the source DB.
        /// When working with read replicas created in a different region, defaults to the `default` Subnet Group.
        /// See [DBSubnetGroupName in API action CreateDBInstanceReadReplica](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_CreateDBInstanceReadReplica.html) for additional read replica constraints.
        #[builder(into, default)]
        pub db_subnet_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Use a dedicated log volume (DLV) for the DB instance. Requires Provisioned IOPS. See the [AWS documentation](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_PIOPS.StorageTypes.html#USER_PIOPS.dlv) for more details.
        #[builder(into, default)]
        pub dedicated_log_volume: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether to remove automated backups immediately after the DB instance is deleted. Default is `true`.
        #[builder(into, default)]
        pub delete_automated_backups: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// If the DB instance should have deletion protection enabled. The database can't be deleted when this value is set to `true`. The default is `false`.
        #[builder(into, default)]
        pub deletion_protection: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of the Directory Service Active Directory domain to create the instance in. Conflicts with `domain_fqdn`, `domain_ou`, `domain_auth_secret_arn` and a `domain_dns_ips`.
        #[builder(into, default)]
        pub domain: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN for the Secrets Manager secret with the self managed Active Directory credentials for the user joining the domain. Conflicts with `domain` and `domain_iam_role_name`.
        #[builder(into, default)]
        pub domain_auth_secret_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IPv4 DNS IP addresses of your primary and secondary self managed Active Directory domain controllers. Two IP addresses must be provided. If there isn't a secondary domain controller, use the IP address of the primary domain controller for both entries in the list. Conflicts with `domain` and `domain_iam_role_name`.
        #[builder(into, default)]
        pub domain_dns_ips: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The fully qualified domain name (FQDN) of the self managed Active Directory domain. Conflicts with `domain` and `domain_iam_role_name`.
        #[builder(into, default)]
        pub domain_fqdn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the IAM role to be used when making API calls to the Directory Service. Conflicts with `domain_fqdn`, `domain_ou`, `domain_auth_secret_arn` and a `domain_dns_ips`.
        #[builder(into, default)]
        pub domain_iam_role_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The self managed Active Directory organizational unit for your DB instance to join. Conflicts with `domain` and `domain_iam_role_name`.
        #[builder(into, default)]
        pub domain_ou: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set of log types to enable for exporting to CloudWatch logs. If omitted, no logs will be exported. For supported values, see the EnableCloudwatchLogsExports.member.N parameter in [API action CreateDBInstance](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_CreateDBInstance.html).
        #[builder(into, default)]
        pub enabled_cloudwatch_logs_exports: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The database engine to use. For supported values, see the Engine parameter in [API action CreateDBInstance](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_CreateDBInstance.html). Note that for Amazon Aurora instances the engine must match the DB cluster's engine'. For information on the difference between the available Aurora MySQL engines see [Comparison between Aurora MySQL 1 and Aurora MySQL 2](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/AuroraMySQL.Updates.20180206.html) in the Amazon RDS User Guide.
        #[builder(into, default)]
        pub engine: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The life cycle type for this DB instance. This setting applies only to RDS for MySQL and RDS for PostgreSQL. Valid values are `open-source-rds-extended-support`, `open-source-rds-extended-support-disabled`. Default value is `open-source-rds-extended-support`. [Using Amazon RDS Extended Support]: https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/extended-support.html
        #[builder(into, default)]
        pub engine_lifecycle_support: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The engine version to use. If `auto_minor_version_upgrade` is enabled, you can provide a prefix of the version such as `8.0` (for `8.0.36`). The actual engine version used is returned in the attribute `engine_version_actual`, see Attribute Reference below. For supported values, see the EngineVersion parameter in [API action CreateDBInstance](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_CreateDBInstance.html). Note that for Amazon Aurora instances the engine version must match the DB cluster's engine version'.
        #[builder(into, default)]
        pub engine_version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of your final DB snapshot
        /// when this DB instance is deleted. Must be provided if `skip_final_snapshot` is
        /// set to `false`. The value must begin with a letter, only contain alphanumeric characters and hyphens, and not end with a hyphen or contain two consecutive hyphens. Must not be provided when deleting a read replica.
        #[builder(into, default)]
        pub final_snapshot_identifier: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Specifies whether mappings of AWS Identity and Access Management (IAM) accounts to database
        /// accounts is enabled.
        #[builder(into, default)]
        pub iam_database_authentication_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The name of the RDS instance, if omitted, this provider will assign a random, unique identifier. Required if `restore_to_point_in_time` is specified.
        #[builder(into, default)]
        pub identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Creates a unique identifier beginning with the specified prefix. Conflicts with `identifier`.
        #[builder(into, default)]
        pub identifier_prefix: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The instance type of the RDS instance.
        #[builder(into)]
        pub instance_class: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The amount of provisioned IOPS. Setting this implies a
        /// storage_type of "io1" or "io2". Can only be set when `storage_type` is `"io1"`, `"io2` or `"gp3"`.
        /// Cannot be specified for gp3 storage if the `allocated_storage` value is below a per-`engine` threshold.
        /// See the [RDS User Guide](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_Storage.html#gp3-storage) for details.
        #[builder(into, default)]
        pub iops: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ARN for the KMS encryption key. If creating an
        /// encrypted replica, set this to the destination KMS ARN.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// License model information for this DB instance. Valid values for this field are as follows:
        /// * RDS for MariaDB: `general-public-license`
        /// * RDS for Microsoft SQL Server: `license-included`
        /// * RDS for MySQL: `general-public-license`
        /// * RDS for Oracle: `bring-your-own-license | license-included`
        /// * RDS for PostgreSQL: `postgresql-license`
        #[builder(into, default)]
        pub license_model: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The window to perform maintenance in.
        /// Syntax: "ddd:hh24:mi-ddd:hh24:mi". Eg: "Mon:00:00-Mon:03:00". See [RDS
        /// Maintenance Window
        /// docs](http://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_UpgradeDBInstance.Maintenance.html#AdjustingTheMaintenanceWindow)
        /// for more information.
        #[builder(into, default)]
        pub maintenance_window: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Set to true to allow RDS to manage the master user password in Secrets Manager. Cannot be set if `password` is provided.
        #[builder(into, default)]
        pub manage_master_user_password: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key. To use a KMS key in a different Amazon Web Services account, specify the key ARN or alias ARN. If not specified, the default KMS key for your Amazon Web Services account is used.
        #[builder(into, default)]
        pub master_user_secret_kms_key_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// When configured, the upper limit to which Amazon RDS can automatically scale the storage of the DB instance. Configuring this will automatically ignore differences to `allocated_storage`. Must be greater than or equal to `allocated_storage` or `0` to disable Storage Autoscaling.
        #[builder(into, default)]
        pub max_allocated_storage: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The interval, in seconds, between points
        /// when Enhanced Monitoring metrics are collected for the DB instance. To disable
        /// collecting Enhanced Monitoring metrics, specify 0. The default is 0. Valid
        /// Values: 0, 1, 5, 10, 15, 30, 60.
        #[builder(into, default)]
        pub monitoring_interval: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ARN for the IAM role that permits RDS
        /// to send enhanced monitoring metrics to CloudWatch Logs. You can find more
        /// information on the [AWS
        /// Documentation](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Monitoring.html)
        /// what IAM permissions are needed to allow Enhanced Monitoring for RDS Instances.
        #[builder(into, default)]
        pub monitoring_role_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies if the RDS instance is multi-AZ
        #[builder(into, default)]
        pub multi_az: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The national character set is used in the NCHAR, NVARCHAR2, and NCLOB data types for Oracle instances. This can't be changed. See [Oracle Character Sets
        /// Supported in Amazon RDS](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Appendix.OracleCharacterSets.html).
        #[builder(into, default)]
        pub nchar_character_set_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The network type of the DB instance. Valid values: `IPV4`, `DUAL`.
        #[builder(into, default)]
        pub network_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the DB option group to associate.
        #[builder(into, default)]
        pub option_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the DB parameter group to associate.
        #[builder(into, default)]
        pub parameter_group_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// (Required unless `manage_master_user_password` is set to true or unless a `snapshot_identifier` or `replicate_source_db`
        /// is provided or `manage_master_user_password` is set.) Password for the master DB user. Note that this may show up in
        /// logs, and it will be stored in the state file. Cannot be set if `manage_master_user_password` is set to `true`.
        #[builder(into, default)]
        pub password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether Performance Insights are enabled. Defaults to false.
        #[builder(into, default)]
        pub performance_insights_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The ARN for the KMS key to encrypt Performance Insights data. When specifying `performance_insights_kms_key_id`, `performance_insights_enabled` needs to be set to true. Once KMS key is set, it can never be changed.
        #[builder(into, default)]
        pub performance_insights_kms_key_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Amount of time in days to retain Performance Insights data. Valid values are `7`, `731` (2 years) or a multiple of `31`. When specifying `performance_insights_retention_period`, `performance_insights_enabled` needs to be set to true. Defaults to '7'.
        #[builder(into, default)]
        pub performance_insights_retention_period: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// The port on which the DB accepts connections.
        #[builder(into, default)]
        pub port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Bool to control if instance is publicly
        /// accessible. Default is `false`.
        #[builder(into, default)]
        pub publicly_accessible: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether the replica is in either `mounted` or `open-read-only` mode. This attribute
        /// is only supported by Oracle instances. Oracle replicas operate in `open-read-only` mode unless otherwise specified. See [Working with Oracle Read Replicas](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/oracle-read-replicas.html) for more information.
        #[builder(into, default)]
        pub replica_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies that this resource is a Replica database, and to use this value as the source database.
        /// If replicating an Amazon RDS Database Instance in the same region, use the `identifier` of the source DB, unless also specifying the `db_subnet_group_name`.
        /// If specifying the `db_subnet_group_name` in the same region, use the `arn` of the source DB.
        /// If replicating an Instance in a different region, use the `arn` of the source DB.
        /// Note that if you are creating a cross-region replica of an encrypted database you will also need to specify a `kms_key_id`.
        /// See [DB Instance Replication][instance-replication] and [Working with PostgreSQL and MySQL Read Replicas](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_ReadRepl.html) for more information on using Replication.
        #[builder(into, default)]
        pub replicate_source_db: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A configuration block for restoring a DB instance to an arbitrary point in time.
        /// Requires the `identifier` argument to be set with the name of the new DB instance to be created.
        /// See Restore To Point In Time below for details.
        #[builder(into, default)]
        pub restore_to_point_in_time: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rds::InstanceRestoreToPointInTime>,
        >,
        /// Restore from a Percona Xtrabackup in S3.  See [Importing Data into an Amazon RDS MySQL DB Instance](http://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/MySQL.Procedural.Importing.html)
        #[builder(into, default)]
        pub s3_import: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rds::InstanceS3Import>,
        >,
        /// Determines whether a final DB snapshot is
        /// created before the DB instance is deleted. If true is specified, no DBSnapshot
        /// is created. If false is specified, a DB snapshot is created before the DB
        /// instance is deleted, using the value from `final_snapshot_identifier`. Default
        /// is `false`.
        #[builder(into, default)]
        pub skip_final_snapshot: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies whether or not to create this database from a snapshot.
        /// This corresponds to the snapshot ID you'd find in the RDS console, e.g: rds:production-2015-06-26-06-05.
        #[builder(into, default)]
        pub snapshot_identifier: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies whether the DB instance is
        /// encrypted. Note that if you are creating a cross-region read replica this field
        /// is ignored and you should instead declare `kms_key_id` with a valid ARN. The
        /// default is `false` if not specified.
        #[builder(into, default)]
        pub storage_encrypted: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The storage throughput value for the DB instance. Can only be set when `storage_type` is `"gp3"`. Cannot be specified if the `allocated_storage` value is below a per-`engine` threshold. See the [RDS User Guide](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_Storage.html#gp3-storage) for details.
        #[builder(into, default)]
        pub storage_throughput: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// One of "standard" (magnetic), "gp2" (general
        /// purpose SSD), "gp3" (general purpose SSD that needs `iops` independently)
        /// "io1" (provisioned IOPS SSD) or "io2" (block express storage provisioned IOPS
        /// SSD). The default is "io1" if `iops` is specified, "gp2" if not.
        #[builder(into, default)]
        pub storage_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Time zone of the DB instance. `timezone` is currently
        /// only supported by Microsoft SQL Server. The `timezone` can only be set on
        /// creation. See [MSSQL User
        /// Guide](http://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_SQLServer.html#SQLServer.Concepts.General.TimeZone)
        /// for more information.
        #[builder(into, default)]
        pub timezone: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to upgrade the storage file system configuration on the read replica.
        /// Can only be set with `replicate_source_db`.
        #[builder(into, default)]
        pub upgrade_storage_config: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// (Required unless a `snapshot_identifier` or `replicate_source_db`
        /// is provided) Username for the master DB user. Cannot be specified for a replica.
        #[builder(into, default)]
        pub username: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of VPC security groups to
        /// associate.
        #[builder(into, default)]
        pub vpc_security_group_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct InstanceResult {
        /// Specifies the DNS address of the DB instance.
        pub address: pulumi_gestalt_rust::Output<String>,
        /// The allocated storage in gibibytes. If `max_allocated_storage` is configured, this argument represents the initial storage allocation and differences from the configuration will be ignored automatically when Storage Autoscaling occurs. If `replicate_source_db` is set, the value is ignored during the creation of the instance.
        pub allocated_storage: pulumi_gestalt_rust::Output<i32>,
        /// Indicates that major version
        /// upgrades are allowed. Changing this parameter does not result in an outage and
        /// the change is asynchronously applied as soon as possible.
        pub allow_major_version_upgrade: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether any database modifications
        /// are applied immediately, or during the next maintenance window. Default is
        /// `false`. See [Amazon RDS Documentation for more
        /// information.](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Overview.DBInstance.Modifying.html)
        pub apply_immediately: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ARN of the RDS instance.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Indicates that minor engine upgrades
        /// will be applied automatically to the DB instance during the maintenance window.
        /// Defaults to true.
        pub auto_minor_version_upgrade: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The AZ for the RDS instance.
        pub availability_zone: pulumi_gestalt_rust::Output<String>,
        /// The days to retain backups for.
        /// Must be between `0` and `35`.
        /// Default is `0`.
        /// Must be greater than `0` if the database is used as a source for a [Read Replica][instance-replication],
        /// uses low-downtime updates,
        /// or will use [RDS Blue/Green deployments][blue-green].
        pub backup_retention_period: pulumi_gestalt_rust::Output<i32>,
        /// Specifies where automated backups and manual snapshots are stored. Possible values are `region` (default) and `outposts`. See [Working with Amazon RDS on AWS Outposts](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/rds-on-outposts.html) for more information.
        pub backup_target: pulumi_gestalt_rust::Output<String>,
        /// The daily time range (in UTC) during which automated backups are created if they are enabled.
        /// Example: "09:46-10:16". Must not overlap with `maintenance_window`.
        pub backup_window: pulumi_gestalt_rust::Output<String>,
        /// Enables low-downtime updates using [RDS Blue/Green deployments][blue-green].
        /// See `blue_green_update` below.
        pub blue_green_update: pulumi_gestalt_rust::Output<
            Option<super::super::types::rds::InstanceBlueGreenUpdate>,
        >,
        /// The identifier of the CA certificate for the DB instance.
        pub ca_cert_identifier: pulumi_gestalt_rust::Output<String>,
        /// The character set name to use for DB encoding in Oracle and Microsoft SQL instances (collation).
        /// This can't be changed.
        /// See [Oracle Character Sets Supported in Amazon RDS](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Appendix.OracleCharacterSets.html) or
        /// [Server-Level Collation for Microsoft SQL Server](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Appendix.SQLServer.CommonDBATasks.Collation.html) for more information.
        /// Cannot be set  with `replicate_source_db`, `restore_to_point_in_time`, `s3_import`, or `snapshot_identifier`.
        pub character_set_name: pulumi_gestalt_rust::Output<String>,
        /// Copy all Instance `tags` to snapshots. Default is `false`.
        pub copy_tags_to_snapshot: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The instance profile associated with the underlying Amazon EC2 instance of an RDS Custom DB instance.
        pub custom_iam_instance_profile: pulumi_gestalt_rust::Output<Option<String>>,
        /// Indicates whether to enable a customer-owned IP address (CoIP) for an RDS on Outposts DB instance. See [CoIP for RDS on Outposts](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/rds-on-outposts.html#rds-on-outposts.coip) for more information.
        ///
        /// > **NOTE:** Removing the `replicate_source_db` attribute from an existing RDS
        /// Replicate database managed by the provider will promote the database to a fully
        /// standalone database.
        pub customer_owned_ip_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the database to create when the DB instance is created. If this parameter is not specified, no database is created in the DB instance. Note that this does not apply for Oracle or SQL Server engines. See the [AWS documentation](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/rds/create-db-instance.html) for more details on what applies for those engines. If you are providing an Oracle db name, it needs to be in all upper case. Cannot be specified for a replica.
        pub db_name: pulumi_gestalt_rust::Output<String>,
        /// Name of DB subnet group.
        /// DB instance will be created in the VPC associated with the DB subnet group.
        /// If unspecified, will be created in the `default` Subnet Group.
        /// When working with read replicas created in the same region, defaults to the Subnet Group Name of the source DB.
        /// When working with read replicas created in a different region, defaults to the `default` Subnet Group.
        /// See [DBSubnetGroupName in API action CreateDBInstanceReadReplica](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_CreateDBInstanceReadReplica.html) for additional read replica constraints.
        pub db_subnet_group_name: pulumi_gestalt_rust::Output<String>,
        /// Use a dedicated log volume (DLV) for the DB instance. Requires Provisioned IOPS. See the [AWS documentation](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_PIOPS.StorageTypes.html#USER_PIOPS.dlv) for more details.
        pub dedicated_log_volume: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether to remove automated backups immediately after the DB instance is deleted. Default is `true`.
        pub delete_automated_backups: pulumi_gestalt_rust::Output<Option<bool>>,
        /// If the DB instance should have deletion protection enabled. The database can't be deleted when this value is set to `true`. The default is `false`.
        pub deletion_protection: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ID of the Directory Service Active Directory domain to create the instance in. Conflicts with `domain_fqdn`, `domain_ou`, `domain_auth_secret_arn` and a `domain_dns_ips`.
        pub domain: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ARN for the Secrets Manager secret with the self managed Active Directory credentials for the user joining the domain. Conflicts with `domain` and `domain_iam_role_name`.
        pub domain_auth_secret_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IPv4 DNS IP addresses of your primary and secondary self managed Active Directory domain controllers. Two IP addresses must be provided. If there isn't a secondary domain controller, use the IP address of the primary domain controller for both entries in the list. Conflicts with `domain` and `domain_iam_role_name`.
        pub domain_dns_ips: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The fully qualified domain name (FQDN) of the self managed Active Directory domain. Conflicts with `domain` and `domain_iam_role_name`.
        pub domain_fqdn: pulumi_gestalt_rust::Output<String>,
        /// The name of the IAM role to be used when making API calls to the Directory Service. Conflicts with `domain_fqdn`, `domain_ou`, `domain_auth_secret_arn` and a `domain_dns_ips`.
        pub domain_iam_role_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The self managed Active Directory organizational unit for your DB instance to join. Conflicts with `domain` and `domain_iam_role_name`.
        pub domain_ou: pulumi_gestalt_rust::Output<Option<String>>,
        /// Set of log types to enable for exporting to CloudWatch logs. If omitted, no logs will be exported. For supported values, see the EnableCloudwatchLogsExports.member.N parameter in [API action CreateDBInstance](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_CreateDBInstance.html).
        pub enabled_cloudwatch_logs_exports: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// The connection endpoint in `address:port` format.
        pub endpoint: pulumi_gestalt_rust::Output<String>,
        /// The database engine to use. For supported values, see the Engine parameter in [API action CreateDBInstance](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_CreateDBInstance.html). Note that for Amazon Aurora instances the engine must match the DB cluster's engine'. For information on the difference between the available Aurora MySQL engines see [Comparison between Aurora MySQL 1 and Aurora MySQL 2](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/AuroraMySQL.Updates.20180206.html) in the Amazon RDS User Guide.
        pub engine: pulumi_gestalt_rust::Output<String>,
        /// The life cycle type for this DB instance. This setting applies only to RDS for MySQL and RDS for PostgreSQL. Valid values are `open-source-rds-extended-support`, `open-source-rds-extended-support-disabled`. Default value is `open-source-rds-extended-support`. [Using Amazon RDS Extended Support]: https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/extended-support.html
        pub engine_lifecycle_support: pulumi_gestalt_rust::Output<String>,
        /// The engine version to use. If `auto_minor_version_upgrade` is enabled, you can provide a prefix of the version such as `8.0` (for `8.0.36`). The actual engine version used is returned in the attribute `engine_version_actual`, see Attribute Reference below. For supported values, see the EngineVersion parameter in [API action CreateDBInstance](https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_CreateDBInstance.html). Note that for Amazon Aurora instances the engine version must match the DB cluster's engine version'.
        pub engine_version: pulumi_gestalt_rust::Output<String>,
        /// The running version of the database.
        pub engine_version_actual: pulumi_gestalt_rust::Output<String>,
        /// The name of your final DB snapshot
        /// when this DB instance is deleted. Must be provided if `skip_final_snapshot` is
        /// set to `false`. The value must begin with a letter, only contain alphanumeric characters and hyphens, and not end with a hyphen or contain two consecutive hyphens. Must not be provided when deleting a read replica.
        pub final_snapshot_identifier: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the ID that Amazon Route 53 assigns when you create a hosted zone.
        pub hosted_zone_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether mappings of AWS Identity and Access Management (IAM) accounts to database
        /// accounts is enabled.
        pub iam_database_authentication_enabled: pulumi_gestalt_rust::Output<
            Option<bool>,
        >,
        /// The name of the RDS instance, if omitted, this provider will assign a random, unique identifier. Required if `restore_to_point_in_time` is specified.
        pub identifier: pulumi_gestalt_rust::Output<String>,
        /// Creates a unique identifier beginning with the specified prefix. Conflicts with `identifier`.
        pub identifier_prefix: pulumi_gestalt_rust::Output<String>,
        /// The instance type of the RDS instance.
        pub instance_class: pulumi_gestalt_rust::Output<String>,
        /// The amount of provisioned IOPS. Setting this implies a
        /// storage_type of "io1" or "io2". Can only be set when `storage_type` is `"io1"`, `"io2` or `"gp3"`.
        /// Cannot be specified for gp3 storage if the `allocated_storage` value is below a per-`engine` threshold.
        /// See the [RDS User Guide](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_Storage.html#gp3-storage) for details.
        pub iops: pulumi_gestalt_rust::Output<i32>,
        /// The ARN for the KMS encryption key. If creating an
        /// encrypted replica, set this to the destination KMS ARN.
        pub kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// The latest time, in UTC [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8), to which a database can be restored with point-in-time restore.
        pub latest_restorable_time: pulumi_gestalt_rust::Output<String>,
        /// License model information for this DB instance. Valid values for this field are as follows:
        /// * RDS for MariaDB: `general-public-license`
        /// * RDS for Microsoft SQL Server: `license-included`
        /// * RDS for MySQL: `general-public-license`
        /// * RDS for Oracle: `bring-your-own-license | license-included`
        /// * RDS for PostgreSQL: `postgresql-license`
        pub license_model: pulumi_gestalt_rust::Output<String>,
        /// Specifies the listener connection endpoint for SQL Server Always On. See endpoint below.
        pub listener_endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::types::rds::InstanceListenerEndpoint>,
        >,
        /// The window to perform maintenance in.
        /// Syntax: "ddd:hh24:mi-ddd:hh24:mi". Eg: "Mon:00:00-Mon:03:00". See [RDS
        /// Maintenance Window
        /// docs](http://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_UpgradeDBInstance.Maintenance.html#AdjustingTheMaintenanceWindow)
        /// for more information.
        pub maintenance_window: pulumi_gestalt_rust::Output<String>,
        /// Set to true to allow RDS to manage the master user password in Secrets Manager. Cannot be set if `password` is provided.
        pub manage_master_user_password: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Amazon Web Services KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key. To use a KMS key in a different Amazon Web Services account, specify the key ARN or alias ARN. If not specified, the default KMS key for your Amazon Web Services account is used.
        pub master_user_secret_kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// A block that specifies the master user secret. Only available when `manage_master_user_password` is set to true. Documented below.
        pub master_user_secrets: pulumi_gestalt_rust::Output<
            Vec<super::super::types::rds::InstanceMasterUserSecret>,
        >,
        /// When configured, the upper limit to which Amazon RDS can automatically scale the storage of the DB instance. Configuring this will automatically ignore differences to `allocated_storage`. Must be greater than or equal to `allocated_storage` or `0` to disable Storage Autoscaling.
        pub max_allocated_storage: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The interval, in seconds, between points
        /// when Enhanced Monitoring metrics are collected for the DB instance. To disable
        /// collecting Enhanced Monitoring metrics, specify 0. The default is 0. Valid
        /// Values: 0, 1, 5, 10, 15, 30, 60.
        pub monitoring_interval: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ARN for the IAM role that permits RDS
        /// to send enhanced monitoring metrics to CloudWatch Logs. You can find more
        /// information on the [AWS
        /// Documentation](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Monitoring.html)
        /// what IAM permissions are needed to allow Enhanced Monitoring for RDS Instances.
        pub monitoring_role_arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies if the RDS instance is multi-AZ
        pub multi_az: pulumi_gestalt_rust::Output<bool>,
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The national character set is used in the NCHAR, NVARCHAR2, and NCLOB data types for Oracle instances. This can't be changed. See [Oracle Character Sets
        /// Supported in Amazon RDS](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Appendix.OracleCharacterSets.html).
        pub nchar_character_set_name: pulumi_gestalt_rust::Output<String>,
        /// The network type of the DB instance. Valid values: `IPV4`, `DUAL`.
        pub network_type: pulumi_gestalt_rust::Output<String>,
        /// Name of the DB option group to associate.
        pub option_group_name: pulumi_gestalt_rust::Output<String>,
        /// Name of the DB parameter group to associate.
        pub parameter_group_name: pulumi_gestalt_rust::Output<String>,
        /// (Required unless `manage_master_user_password` is set to true or unless a `snapshot_identifier` or `replicate_source_db`
        /// is provided or `manage_master_user_password` is set.) Password for the master DB user. Note that this may show up in
        /// logs, and it will be stored in the state file. Cannot be set if `manage_master_user_password` is set to `true`.
        pub password: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies whether Performance Insights are enabled. Defaults to false.
        pub performance_insights_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ARN for the KMS key to encrypt Performance Insights data. When specifying `performance_insights_kms_key_id`, `performance_insights_enabled` needs to be set to true. Once KMS key is set, it can never be changed.
        pub performance_insights_kms_key_id: pulumi_gestalt_rust::Output<String>,
        /// Amount of time in days to retain Performance Insights data. Valid values are `7`, `731` (2 years) or a multiple of `31`. When specifying `performance_insights_retention_period`, `performance_insights_enabled` needs to be set to true. Defaults to '7'.
        pub performance_insights_retention_period: pulumi_gestalt_rust::Output<i32>,
        /// The port on which the DB accepts connections.
        pub port: pulumi_gestalt_rust::Output<i32>,
        /// Bool to control if instance is publicly
        /// accessible. Default is `false`.
        pub publicly_accessible: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether the replica is in either `mounted` or `open-read-only` mode. This attribute
        /// is only supported by Oracle instances. Oracle replicas operate in `open-read-only` mode unless otherwise specified. See [Working with Oracle Read Replicas](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/oracle-read-replicas.html) for more information.
        pub replica_mode: pulumi_gestalt_rust::Output<String>,
        pub replicas: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Specifies that this resource is a Replica database, and to use this value as the source database.
        /// If replicating an Amazon RDS Database Instance in the same region, use the `identifier` of the source DB, unless also specifying the `db_subnet_group_name`.
        /// If specifying the `db_subnet_group_name` in the same region, use the `arn` of the source DB.
        /// If replicating an Instance in a different region, use the `arn` of the source DB.
        /// Note that if you are creating a cross-region replica of an encrypted database you will also need to specify a `kms_key_id`.
        /// See [DB Instance Replication][instance-replication] and [Working with PostgreSQL and MySQL Read Replicas](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_ReadRepl.html) for more information on using Replication.
        pub replicate_source_db: pulumi_gestalt_rust::Output<Option<String>>,
        /// The RDS Resource ID of this instance.
        pub resource_id: pulumi_gestalt_rust::Output<String>,
        /// A configuration block for restoring a DB instance to an arbitrary point in time.
        /// Requires the `identifier` argument to be set with the name of the new DB instance to be created.
        /// See Restore To Point In Time below for details.
        pub restore_to_point_in_time: pulumi_gestalt_rust::Output<
            Option<super::super::types::rds::InstanceRestoreToPointInTime>,
        >,
        /// Restore from a Percona Xtrabackup in S3.  See [Importing Data into an Amazon RDS MySQL DB Instance](http://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/MySQL.Procedural.Importing.html)
        pub s3_import: pulumi_gestalt_rust::Output<
            Option<super::super::types::rds::InstanceS3Import>,
        >,
        /// Determines whether a final DB snapshot is
        /// created before the DB instance is deleted. If true is specified, no DBSnapshot
        /// is created. If false is specified, a DB snapshot is created before the DB
        /// instance is deleted, using the value from `final_snapshot_identifier`. Default
        /// is `false`.
        pub skip_final_snapshot: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies whether or not to create this database from a snapshot.
        /// This corresponds to the snapshot ID you'd find in the RDS console, e.g: rds:production-2015-06-26-06-05.
        pub snapshot_identifier: pulumi_gestalt_rust::Output<String>,
        /// The RDS instance status.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether the DB instance is
        /// encrypted. Note that if you are creating a cross-region read replica this field
        /// is ignored and you should instead declare `kms_key_id` with a valid ARN. The
        /// default is `false` if not specified.
        pub storage_encrypted: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The storage throughput value for the DB instance. Can only be set when `storage_type` is `"gp3"`. Cannot be specified if the `allocated_storage` value is below a per-`engine` threshold. See the [RDS User Guide](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_Storage.html#gp3-storage) for details.
        pub storage_throughput: pulumi_gestalt_rust::Output<i32>,
        /// One of "standard" (magnetic), "gp2" (general
        /// purpose SSD), "gp3" (general purpose SSD that needs `iops` independently)
        /// "io1" (provisioned IOPS SSD) or "io2" (block express storage provisioned IOPS
        /// SSD). The default is "io1" if `iops` is specified, "gp2" if not.
        pub storage_type: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Time zone of the DB instance. `timezone` is currently
        /// only supported by Microsoft SQL Server. The `timezone` can only be set on
        /// creation. See [MSSQL User
        /// Guide](http://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_SQLServer.html#SQLServer.Concepts.General.TimeZone)
        /// for more information.
        pub timezone: pulumi_gestalt_rust::Output<String>,
        /// Whether to upgrade the storage file system configuration on the read replica.
        /// Can only be set with `replicate_source_db`.
        pub upgrade_storage_config: pulumi_gestalt_rust::Output<Option<bool>>,
        /// (Required unless a `snapshot_identifier` or `replicate_source_db`
        /// is provided) Username for the master DB user. Cannot be specified for a replica.
        pub username: pulumi_gestalt_rust::Output<String>,
        /// List of VPC security groups to
        /// associate.
        pub vpc_security_group_ids: pulumi_gestalt_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: InstanceArgs,
    ) -> InstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let allocated_storage_binding = args
            .allocated_storage
            .get_output(context)
            .get_inner();
        let allow_major_version_upgrade_binding = args
            .allow_major_version_upgrade
            .get_output(context)
            .get_inner();
        let apply_immediately_binding = args
            .apply_immediately
            .get_output(context)
            .get_inner();
        let auto_minor_version_upgrade_binding = args
            .auto_minor_version_upgrade
            .get_output(context)
            .get_inner();
        let availability_zone_binding = args
            .availability_zone
            .get_output(context)
            .get_inner();
        let backup_retention_period_binding = args
            .backup_retention_period
            .get_output(context)
            .get_inner();
        let backup_target_binding = args.backup_target.get_output(context).get_inner();
        let backup_window_binding = args.backup_window.get_output(context).get_inner();
        let blue_green_update_binding = args
            .blue_green_update
            .get_output(context)
            .get_inner();
        let ca_cert_identifier_binding = args
            .ca_cert_identifier
            .get_output(context)
            .get_inner();
        let character_set_name_binding = args
            .character_set_name
            .get_output(context)
            .get_inner();
        let copy_tags_to_snapshot_binding = args
            .copy_tags_to_snapshot
            .get_output(context)
            .get_inner();
        let custom_iam_instance_profile_binding = args
            .custom_iam_instance_profile
            .get_output(context)
            .get_inner();
        let customer_owned_ip_enabled_binding = args
            .customer_owned_ip_enabled
            .get_output(context)
            .get_inner();
        let db_name_binding = args.db_name.get_output(context).get_inner();
        let db_subnet_group_name_binding = args
            .db_subnet_group_name
            .get_output(context)
            .get_inner();
        let dedicated_log_volume_binding = args
            .dedicated_log_volume
            .get_output(context)
            .get_inner();
        let delete_automated_backups_binding = args
            .delete_automated_backups
            .get_output(context)
            .get_inner();
        let deletion_protection_binding = args
            .deletion_protection
            .get_output(context)
            .get_inner();
        let domain_binding = args.domain.get_output(context).get_inner();
        let domain_auth_secret_arn_binding = args
            .domain_auth_secret_arn
            .get_output(context)
            .get_inner();
        let domain_dns_ips_binding = args.domain_dns_ips.get_output(context).get_inner();
        let domain_fqdn_binding = args.domain_fqdn.get_output(context).get_inner();
        let domain_iam_role_name_binding = args
            .domain_iam_role_name
            .get_output(context)
            .get_inner();
        let domain_ou_binding = args.domain_ou.get_output(context).get_inner();
        let enabled_cloudwatch_logs_exports_binding = args
            .enabled_cloudwatch_logs_exports
            .get_output(context)
            .get_inner();
        let engine_binding = args.engine.get_output(context).get_inner();
        let engine_lifecycle_support_binding = args
            .engine_lifecycle_support
            .get_output(context)
            .get_inner();
        let engine_version_binding = args.engine_version.get_output(context).get_inner();
        let final_snapshot_identifier_binding = args
            .final_snapshot_identifier
            .get_output(context)
            .get_inner();
        let iam_database_authentication_enabled_binding = args
            .iam_database_authentication_enabled
            .get_output(context)
            .get_inner();
        let identifier_binding = args.identifier.get_output(context).get_inner();
        let identifier_prefix_binding = args
            .identifier_prefix
            .get_output(context)
            .get_inner();
        let instance_class_binding = args.instance_class.get_output(context).get_inner();
        let iops_binding = args.iops.get_output(context).get_inner();
        let kms_key_id_binding = args.kms_key_id.get_output(context).get_inner();
        let license_model_binding = args.license_model.get_output(context).get_inner();
        let maintenance_window_binding = args
            .maintenance_window
            .get_output(context)
            .get_inner();
        let manage_master_user_password_binding = args
            .manage_master_user_password
            .get_output(context)
            .get_inner();
        let master_user_secret_kms_key_id_binding = args
            .master_user_secret_kms_key_id
            .get_output(context)
            .get_inner();
        let max_allocated_storage_binding = args
            .max_allocated_storage
            .get_output(context)
            .get_inner();
        let monitoring_interval_binding = args
            .monitoring_interval
            .get_output(context)
            .get_inner();
        let monitoring_role_arn_binding = args
            .monitoring_role_arn
            .get_output(context)
            .get_inner();
        let multi_az_binding = args.multi_az.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let nchar_character_set_name_binding = args
            .nchar_character_set_name
            .get_output(context)
            .get_inner();
        let network_type_binding = args.network_type.get_output(context).get_inner();
        let option_group_name_binding = args
            .option_group_name
            .get_output(context)
            .get_inner();
        let parameter_group_name_binding = args
            .parameter_group_name
            .get_output(context)
            .get_inner();
        let password_binding = args.password.get_output(context).get_inner();
        let performance_insights_enabled_binding = args
            .performance_insights_enabled
            .get_output(context)
            .get_inner();
        let performance_insights_kms_key_id_binding = args
            .performance_insights_kms_key_id
            .get_output(context)
            .get_inner();
        let performance_insights_retention_period_binding = args
            .performance_insights_retention_period
            .get_output(context)
            .get_inner();
        let port_binding = args.port.get_output(context).get_inner();
        let publicly_accessible_binding = args
            .publicly_accessible
            .get_output(context)
            .get_inner();
        let replica_mode_binding = args.replica_mode.get_output(context).get_inner();
        let replicate_source_db_binding = args
            .replicate_source_db
            .get_output(context)
            .get_inner();
        let restore_to_point_in_time_binding = args
            .restore_to_point_in_time
            .get_output(context)
            .get_inner();
        let s3_import_binding = args.s3_import.get_output(context).get_inner();
        let skip_final_snapshot_binding = args
            .skip_final_snapshot
            .get_output(context)
            .get_inner();
        let snapshot_identifier_binding = args
            .snapshot_identifier
            .get_output(context)
            .get_inner();
        let storage_encrypted_binding = args
            .storage_encrypted
            .get_output(context)
            .get_inner();
        let storage_throughput_binding = args
            .storage_throughput
            .get_output(context)
            .get_inner();
        let storage_type_binding = args.storage_type.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let timezone_binding = args.timezone.get_output(context).get_inner();
        let upgrade_storage_config_binding = args
            .upgrade_storage_config
            .get_output(context)
            .get_inner();
        let username_binding = args.username.get_output(context).get_inner();
        let vpc_security_group_ids_binding = args
            .vpc_security_group_ids
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rds/instance:Instance".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "allocatedStorage".into(),
                    value: &allocated_storage_binding,
                },
                register_interface::ObjectField {
                    name: "allowMajorVersionUpgrade".into(),
                    value: &allow_major_version_upgrade_binding,
                },
                register_interface::ObjectField {
                    name: "applyImmediately".into(),
                    value: &apply_immediately_binding,
                },
                register_interface::ObjectField {
                    name: "autoMinorVersionUpgrade".into(),
                    value: &auto_minor_version_upgrade_binding,
                },
                register_interface::ObjectField {
                    name: "availabilityZone".into(),
                    value: &availability_zone_binding,
                },
                register_interface::ObjectField {
                    name: "backupRetentionPeriod".into(),
                    value: &backup_retention_period_binding,
                },
                register_interface::ObjectField {
                    name: "backupTarget".into(),
                    value: &backup_target_binding,
                },
                register_interface::ObjectField {
                    name: "backupWindow".into(),
                    value: &backup_window_binding,
                },
                register_interface::ObjectField {
                    name: "blueGreenUpdate".into(),
                    value: &blue_green_update_binding,
                },
                register_interface::ObjectField {
                    name: "caCertIdentifier".into(),
                    value: &ca_cert_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "characterSetName".into(),
                    value: &character_set_name_binding,
                },
                register_interface::ObjectField {
                    name: "copyTagsToSnapshot".into(),
                    value: &copy_tags_to_snapshot_binding,
                },
                register_interface::ObjectField {
                    name: "customIamInstanceProfile".into(),
                    value: &custom_iam_instance_profile_binding,
                },
                register_interface::ObjectField {
                    name: "customerOwnedIpEnabled".into(),
                    value: &customer_owned_ip_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "dbName".into(),
                    value: &db_name_binding,
                },
                register_interface::ObjectField {
                    name: "dbSubnetGroupName".into(),
                    value: &db_subnet_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "dedicatedLogVolume".into(),
                    value: &dedicated_log_volume_binding,
                },
                register_interface::ObjectField {
                    name: "deleteAutomatedBackups".into(),
                    value: &delete_automated_backups_binding,
                },
                register_interface::ObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding,
                },
                register_interface::ObjectField {
                    name: "domain".into(),
                    value: &domain_binding,
                },
                register_interface::ObjectField {
                    name: "domainAuthSecretArn".into(),
                    value: &domain_auth_secret_arn_binding,
                },
                register_interface::ObjectField {
                    name: "domainDnsIps".into(),
                    value: &domain_dns_ips_binding,
                },
                register_interface::ObjectField {
                    name: "domainFqdn".into(),
                    value: &domain_fqdn_binding,
                },
                register_interface::ObjectField {
                    name: "domainIamRoleName".into(),
                    value: &domain_iam_role_name_binding,
                },
                register_interface::ObjectField {
                    name: "domainOu".into(),
                    value: &domain_ou_binding,
                },
                register_interface::ObjectField {
                    name: "enabledCloudwatchLogsExports".into(),
                    value: &enabled_cloudwatch_logs_exports_binding,
                },
                register_interface::ObjectField {
                    name: "engine".into(),
                    value: &engine_binding,
                },
                register_interface::ObjectField {
                    name: "engineLifecycleSupport".into(),
                    value: &engine_lifecycle_support_binding,
                },
                register_interface::ObjectField {
                    name: "engineVersion".into(),
                    value: &engine_version_binding,
                },
                register_interface::ObjectField {
                    name: "finalSnapshotIdentifier".into(),
                    value: &final_snapshot_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "iamDatabaseAuthenticationEnabled".into(),
                    value: &iam_database_authentication_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "identifier".into(),
                    value: &identifier_binding,
                },
                register_interface::ObjectField {
                    name: "identifierPrefix".into(),
                    value: &identifier_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "instanceClass".into(),
                    value: &instance_class_binding,
                },
                register_interface::ObjectField {
                    name: "iops".into(),
                    value: &iops_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "licenseModel".into(),
                    value: &license_model_binding,
                },
                register_interface::ObjectField {
                    name: "maintenanceWindow".into(),
                    value: &maintenance_window_binding,
                },
                register_interface::ObjectField {
                    name: "manageMasterUserPassword".into(),
                    value: &manage_master_user_password_binding,
                },
                register_interface::ObjectField {
                    name: "masterUserSecretKmsKeyId".into(),
                    value: &master_user_secret_kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "maxAllocatedStorage".into(),
                    value: &max_allocated_storage_binding,
                },
                register_interface::ObjectField {
                    name: "monitoringInterval".into(),
                    value: &monitoring_interval_binding,
                },
                register_interface::ObjectField {
                    name: "monitoringRoleArn".into(),
                    value: &monitoring_role_arn_binding,
                },
                register_interface::ObjectField {
                    name: "multiAz".into(),
                    value: &multi_az_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "ncharCharacterSetName".into(),
                    value: &nchar_character_set_name_binding,
                },
                register_interface::ObjectField {
                    name: "networkType".into(),
                    value: &network_type_binding,
                },
                register_interface::ObjectField {
                    name: "optionGroupName".into(),
                    value: &option_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "parameterGroupName".into(),
                    value: &parameter_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "password".into(),
                    value: &password_binding,
                },
                register_interface::ObjectField {
                    name: "performanceInsightsEnabled".into(),
                    value: &performance_insights_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "performanceInsightsKmsKeyId".into(),
                    value: &performance_insights_kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "performanceInsightsRetentionPeriod".into(),
                    value: &performance_insights_retention_period_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "publiclyAccessible".into(),
                    value: &publicly_accessible_binding,
                },
                register_interface::ObjectField {
                    name: "replicaMode".into(),
                    value: &replica_mode_binding,
                },
                register_interface::ObjectField {
                    name: "replicateSourceDb".into(),
                    value: &replicate_source_db_binding,
                },
                register_interface::ObjectField {
                    name: "restoreToPointInTime".into(),
                    value: &restore_to_point_in_time_binding,
                },
                register_interface::ObjectField {
                    name: "s3Import".into(),
                    value: &s3_import_binding,
                },
                register_interface::ObjectField {
                    name: "skipFinalSnapshot".into(),
                    value: &skip_final_snapshot_binding,
                },
                register_interface::ObjectField {
                    name: "snapshotIdentifier".into(),
                    value: &snapshot_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "storageEncrypted".into(),
                    value: &storage_encrypted_binding,
                },
                register_interface::ObjectField {
                    name: "storageThroughput".into(),
                    value: &storage_throughput_binding,
                },
                register_interface::ObjectField {
                    name: "storageType".into(),
                    value: &storage_type_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "timezone".into(),
                    value: &timezone_binding,
                },
                register_interface::ObjectField {
                    name: "upgradeStorageConfig".into(),
                    value: &upgrade_storage_config_binding,
                },
                register_interface::ObjectField {
                    name: "username".into(),
                    value: &username_binding,
                },
                register_interface::ObjectField {
                    name: "vpcSecurityGroupIds".into(),
                    value: &vpc_security_group_ids_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        InstanceResult {
            address: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("address"),
            ),
            allocated_storage: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allocatedStorage"),
            ),
            allow_major_version_upgrade: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("allowMajorVersionUpgrade"),
            ),
            apply_immediately: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applyImmediately"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            auto_minor_version_upgrade: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoMinorVersionUpgrade"),
            ),
            availability_zone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("availabilityZone"),
            ),
            backup_retention_period: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backupRetentionPeriod"),
            ),
            backup_target: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backupTarget"),
            ),
            backup_window: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("backupWindow"),
            ),
            blue_green_update: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("blueGreenUpdate"),
            ),
            ca_cert_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("caCertIdentifier"),
            ),
            character_set_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("characterSetName"),
            ),
            copy_tags_to_snapshot: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("copyTagsToSnapshot"),
            ),
            custom_iam_instance_profile: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customIamInstanceProfile"),
            ),
            customer_owned_ip_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("customerOwnedIpEnabled"),
            ),
            db_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbName"),
            ),
            db_subnet_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dbSubnetGroupName"),
            ),
            dedicated_log_volume: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dedicatedLogVolume"),
            ),
            delete_automated_backups: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deleteAutomatedBackups"),
            ),
            deletion_protection: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionProtection"),
            ),
            domain: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domain"),
            ),
            domain_auth_secret_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainAuthSecretArn"),
            ),
            domain_dns_ips: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainDnsIps"),
            ),
            domain_fqdn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainFqdn"),
            ),
            domain_iam_role_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainIamRoleName"),
            ),
            domain_ou: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("domainOu"),
            ),
            enabled_cloudwatch_logs_exports: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabledCloudwatchLogsExports"),
            ),
            endpoint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("endpoint"),
            ),
            engine: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engine"),
            ),
            engine_lifecycle_support: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engineLifecycleSupport"),
            ),
            engine_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engineVersion"),
            ),
            engine_version_actual: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("engineVersionActual"),
            ),
            final_snapshot_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("finalSnapshotIdentifier"),
            ),
            hosted_zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("hostedZoneId"),
            ),
            iam_database_authentication_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iamDatabaseAuthenticationEnabled"),
            ),
            identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identifier"),
            ),
            identifier_prefix: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identifierPrefix"),
            ),
            instance_class: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceClass"),
            ),
            iops: pulumi_gestalt_rust::__private::into_domain(o.extract_field("iops")),
            kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            latest_restorable_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("latestRestorableTime"),
            ),
            license_model: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("licenseModel"),
            ),
            listener_endpoints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("listenerEndpoints"),
            ),
            maintenance_window: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maintenanceWindow"),
            ),
            manage_master_user_password: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("manageMasterUserPassword"),
            ),
            master_user_secret_kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("masterUserSecretKmsKeyId"),
            ),
            master_user_secrets: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("masterUserSecrets"),
            ),
            max_allocated_storage: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxAllocatedStorage"),
            ),
            monitoring_interval: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monitoringInterval"),
            ),
            monitoring_role_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("monitoringRoleArn"),
            ),
            multi_az: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("multiAz"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            nchar_character_set_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ncharCharacterSetName"),
            ),
            network_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkType"),
            ),
            option_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("optionGroupName"),
            ),
            parameter_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("parameterGroupName"),
            ),
            password: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("password"),
            ),
            performance_insights_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("performanceInsightsEnabled"),
            ),
            performance_insights_kms_key_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("performanceInsightsKmsKeyId"),
            ),
            performance_insights_retention_period: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("performanceInsightsRetentionPeriod"),
            ),
            port: pulumi_gestalt_rust::__private::into_domain(o.extract_field("port")),
            publicly_accessible: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publiclyAccessible"),
            ),
            replica_mode: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicaMode"),
            ),
            replicas: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicas"),
            ),
            replicate_source_db: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("replicateSourceDb"),
            ),
            resource_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceId"),
            ),
            restore_to_point_in_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("restoreToPointInTime"),
            ),
            s3_import: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("s3Import"),
            ),
            skip_final_snapshot: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skipFinalSnapshot"),
            ),
            snapshot_identifier: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("snapshotIdentifier"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            storage_encrypted: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageEncrypted"),
            ),
            storage_throughput: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageThroughput"),
            ),
            storage_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("storageType"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            timezone: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("timezone"),
            ),
            upgrade_storage_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("upgradeStorageConfig"),
            ),
            username: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("username"),
            ),
            vpc_security_group_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcSecurityGroupIds"),
            ),
        }
    }
}
