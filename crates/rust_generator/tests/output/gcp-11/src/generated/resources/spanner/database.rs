/// A Cloud Spanner Database which is hosted on a Spanner instance.
///
///
/// To get more information about Database, see:
///
/// * [API documentation](https://cloud.google.com/spanner/docs/reference/rest/v1/projects.instances.databases)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/spanner/)
///
/// > **Warning:** On newer versions of the provider, you must explicitly set `deletion_protection=false`
/// (and run `pulumi up` to write the field to state) in order to destroy an instance.
/// It is recommended to not set this field (or set it to true) until you're ready to destroy.
/// On older versions, it is strongly recommended to set `lifecycle { prevent_destroy = true }`
/// on databases in order to prevent accidental data loss.
///
/// ## Example Usage
///
/// ### Spanner Database Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let database = database::create(
///         "database",
///         DatabaseArgs::builder()
///             .ddls(
///                 vec![
///                     "CREATE TABLE t1 (t1 INT64 NOT NULL,) PRIMARY KEY(t1)",
///                     "CREATE TABLE t2 (t2 INT64 NOT NULL,) PRIMARY KEY(t2)",
///                 ],
///             )
///             .deletion_protection(false)
///             .instance("${main.name}")
///             .name("my-database")
///             .version_retention_period("3d")
///             .build_struct(),
///     );
///     let main = instance::create(
///         "main",
///         InstanceArgs::builder()
///             .config("regional-europe-west1")
///             .display_name("main-instance")
///             .num_nodes(1)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Database can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/instances/{{instance}}/databases/{{name}}`
///
/// * `instances/{{instance}}/databases/{{name}}`
///
/// * `{{project}}/{{instance}}/{{name}}`
///
/// * `{{instance}}/{{name}}`
///
/// When using the `pulumi import` command, Database can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:spanner/database:Database default projects/{{project}}/instances/{{instance}}/databases/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:spanner/database:Database default instances/{{instance}}/databases/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:spanner/database:Database default {{project}}/{{instance}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:spanner/database:Database default {{instance}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatabaseArgs {
        /// The dialect of the Cloud Spanner Database.
        /// If it is not provided, "GOOGLE_STANDARD_SQL" will be used.
        /// Possible values are: `GOOGLE_STANDARD_SQL`, `POSTGRESQL`.
        #[builder(into, default)]
        pub database_dialect: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// An optional list of DDL statements to run inside the newly created
        /// database. Statements can create tables, indexes, etc. These statements
        /// execute atomically with the creation of the database: if there is an
        /// error in any statement, the database is not created.
        #[builder(into, default)]
        pub ddls: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Whether or not to allow the provider to destroy the instance. Unless this field is set to false
        /// in state, a `destroy` or `update` that would delete the instance will fail.
        #[builder(into, default)]
        pub deletion_protection: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        #[builder(into, default)]
        pub enable_drop_protection: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Encryption configuration for the database
        /// Structure is documented below.
        #[builder(into, default)]
        pub encryption_config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::spanner::DatabaseEncryptionConfig>,
        >,
        /// The instance to create the database on.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub instance: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A unique identifier for the database, which cannot be changed after the
        /// instance is created. Values are of the form `[a-z][-_a-z0-9]*[a-z0-9]`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The retention period for the database. The retention period must be between 1 hour
        /// and 7 days, and can be specified in days, hours, minutes, or seconds. For example,
        /// the values 1d, 24h, 1440m, and 86400s are equivalent. Default value is 1h.
        /// If this property is used, you must avoid adding new DDL statements to `ddl` that
        /// update the database's version_retention_period.
        #[builder(into, default)]
        pub version_retention_period: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DatabaseResult {
        /// The dialect of the Cloud Spanner Database.
        /// If it is not provided, "GOOGLE_STANDARD_SQL" will be used.
        /// Possible values are: `GOOGLE_STANDARD_SQL`, `POSTGRESQL`.
        pub database_dialect: pulumi_gestalt_rust::Output<String>,
        /// An optional list of DDL statements to run inside the newly created
        /// database. Statements can create tables, indexes, etc. These statements
        /// execute atomically with the creation of the database: if there is an
        /// error in any statement, the database is not created.
        pub ddls: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Whether or not to allow the provider to destroy the instance. Unless this field is set to false
        /// in state, a `destroy` or `update` that would delete the instance will fail.
        pub deletion_protection: pulumi_gestalt_rust::Output<Option<bool>>,
        pub enable_drop_protection: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Encryption configuration for the database
        /// Structure is documented below.
        pub encryption_config: pulumi_gestalt_rust::Output<
            Option<super::super::types::spanner::DatabaseEncryptionConfig>,
        >,
        /// The instance to create the database on.
        ///
        ///
        /// - - -
        pub instance: pulumi_gestalt_rust::Output<String>,
        /// A unique identifier for the database, which cannot be changed after the
        /// instance is created. Values are of the form `[a-z][-_a-z0-9]*[a-z0-9]`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// An explanation of the status of the database.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// The retention period for the database. The retention period must be between 1 hour
        /// and 7 days, and can be specified in days, hours, minutes, or seconds. For example,
        /// the values 1d, 24h, 1440m, and 86400s are equivalent. Default value is 1h.
        /// If this property is used, you must avoid adding new DDL statements to `ddl` that
        /// update the database's version_retention_period.
        pub version_retention_period: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DatabaseArgs,
    ) -> DatabaseResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let database_dialect_binding_1 = args.database_dialect.get_output(context);
        let database_dialect_binding = database_dialect_binding_1.get_inner();
        let ddls_binding_1 = args.ddls.get_output(context);
        let ddls_binding = ddls_binding_1.get_inner();
        let deletion_protection_binding_1 = args.deletion_protection.get_output(context);
        let deletion_protection_binding = deletion_protection_binding_1.get_inner();
        let enable_drop_protection_binding_1 = args
            .enable_drop_protection
            .get_output(context);
        let enable_drop_protection_binding = enable_drop_protection_binding_1
            .get_inner();
        let encryption_config_binding_1 = args.encryption_config.get_output(context);
        let encryption_config_binding = encryption_config_binding_1.get_inner();
        let instance_binding_1 = args.instance.get_output(context);
        let instance_binding = instance_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let project_binding_1 = args.project.get_output(context);
        let project_binding = project_binding_1.get_inner();
        let version_retention_period_binding_1 = args
            .version_retention_period
            .get_output(context);
        let version_retention_period_binding = version_retention_period_binding_1
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:spanner/database:Database".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "databaseDialect".into(),
                    value: &database_dialect_binding,
                },
                register_interface::ObjectField {
                    name: "ddls".into(),
                    value: &ddls_binding,
                },
                register_interface::ObjectField {
                    name: "deletionProtection".into(),
                    value: &deletion_protection_binding,
                },
                register_interface::ObjectField {
                    name: "enableDropProtection".into(),
                    value: &enable_drop_protection_binding,
                },
                register_interface::ObjectField {
                    name: "encryptionConfig".into(),
                    value: &encryption_config_binding,
                },
                register_interface::ObjectField {
                    name: "instance".into(),
                    value: &instance_binding,
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
                    name: "versionRetentionPeriod".into(),
                    value: &version_retention_period_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DatabaseResult {
            database_dialect: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("databaseDialect"),
            ),
            ddls: pulumi_gestalt_rust::__private::into_domain(o.extract_field("ddls")),
            deletion_protection: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("deletionProtection"),
            ),
            enable_drop_protection: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enableDropProtection"),
            ),
            encryption_config: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("encryptionConfig"),
            ),
            instance: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instance"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            state: pulumi_gestalt_rust::__private::into_domain(o.extract_field("state")),
            version_retention_period: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionRetentionPeriod"),
            ),
        }
    }
}
