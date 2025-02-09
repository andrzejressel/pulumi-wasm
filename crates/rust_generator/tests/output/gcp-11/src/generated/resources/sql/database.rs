/// Represents a SQL database inside the Cloud SQL instance, hosted in
/// Google's cloud.
///
///
///
/// ## Example Usage
///
/// ### Sql Database Basic
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
///             .instance("${instance.name}")
///             .name("my-database")
///             .build_struct(),
///     );
///     let instance = database_instance::create(
///         "instance",
///         DatabaseInstanceArgs::builder()
///             .database_version("MYSQL_8_0")
///             .deletion_protection(true)
///             .name("my-database-instance")
///             .region("us-central1")
///             .settings(
///                 DatabaseInstanceSettings::builder().tier("db-f1-micro").build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Sql Database Deletion Policy
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let databaseDeletionPolicy = database::create(
///         "databaseDeletionPolicy",
///         DatabaseArgs::builder()
///             .deletion_policy("ABANDON")
///             .instance("${instance.name}")
///             .name("my-database")
///             .build_struct(),
///     );
///     let instance = database_instance::create(
///         "instance",
///         DatabaseInstanceArgs::builder()
///             .database_version("POSTGRES_14")
///             .deletion_protection(true)
///             .name("my-database-instance")
///             .region("us-central1")
///             .settings(
///                 DatabaseInstanceSettings::builder().tier("db-g1-small").build_struct(),
///             )
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
/// * `{{name}}`
///
/// When using the `pulumi import` command, Database can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:sql/database:Database default projects/{{project}}/instances/{{instance}}/databases/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:sql/database:Database default instances/{{instance}}/databases/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:sql/database:Database default {{project}}/{{instance}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:sql/database:Database default {{instance}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:sql/database:Database default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod database {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatabaseArgs {
        /// The charset value. See MySQL's
        /// [Supported Character Sets and Collations](https://dev.mysql.com/doc/refman/5.7/en/charset-charsets.html)
        /// and Postgres' [Character Set Support](https://www.postgresql.org/docs/9.6/static/multibyte.html)
        /// for more details and supported values. Postgres databases only support
        /// a value of `UTF8` at creation time.
        #[builder(into, default)]
        pub charset: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The collation value. See MySQL's
        /// [Supported Character Sets and Collations](https://dev.mysql.com/doc/refman/5.7/en/charset-charsets.html)
        /// and Postgres' [Collation Support](https://www.postgresql.org/docs/9.6/static/collation.html)
        /// for more details and supported values. Postgres databases only support
        /// a value of `en_US.UTF8` at creation time.
        #[builder(into, default)]
        pub collation: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The deletion policy for the database. Setting ABANDON allows the resource
        /// to be abandoned rather than deleted. This is useful for Postgres, where databases cannot be
        /// deleted from the API if there are users other than cloudsqlsuperuser with access. Possible
        /// values are: "ABANDON", "DELETE". Defaults to "DELETE".
        #[builder(into, default)]
        pub deletion_policy: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Cloud SQL instance. This does not include the project
        /// ID.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub instance: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the database in the Cloud SQL instance.
        /// This does not include the project ID or instance name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DatabaseResult {
        /// The charset value. See MySQL's
        /// [Supported Character Sets and Collations](https://dev.mysql.com/doc/refman/5.7/en/charset-charsets.html)
        /// and Postgres' [Character Set Support](https://www.postgresql.org/docs/9.6/static/multibyte.html)
        /// for more details and supported values. Postgres databases only support
        /// a value of `UTF8` at creation time.
        pub charset: pulumi_gestalt_rust::Output<String>,
        /// The collation value. See MySQL's
        /// [Supported Character Sets and Collations](https://dev.mysql.com/doc/refman/5.7/en/charset-charsets.html)
        /// and Postgres' [Collation Support](https://www.postgresql.org/docs/9.6/static/collation.html)
        /// for more details and supported values. Postgres databases only support
        /// a value of `en_US.UTF8` at creation time.
        pub collation: pulumi_gestalt_rust::Output<String>,
        /// The deletion policy for the database. Setting ABANDON allows the resource
        /// to be abandoned rather than deleted. This is useful for Postgres, where databases cannot be
        /// deleted from the API if there are users other than cloudsqlsuperuser with access. Possible
        /// values are: "ABANDON", "DELETE". Defaults to "DELETE".
        pub deletion_policy: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Cloud SQL instance. This does not include the project
        /// ID.
        ///
        ///
        /// - - -
        pub instance: pulumi_gestalt_rust::Output<String>,
        /// The name of the database in the Cloud SQL instance.
        /// This does not include the project ID or instance name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_gestalt_rust::Output<String>,
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
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let charset_binding = args.charset.get_output(context);
        let collation_binding = args.collation.get_output(context);
        let deletion_policy_binding = args.deletion_policy.get_output(context);
        let instance_binding = args.instance.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:sql/database:Database".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "charset".into(),
                    value: charset_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "collation".into(),
                    value: collation_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deletionPolicy".into(),
                    value: deletion_policy_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instance".into(),
                    value: instance_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        DatabaseResult {
            charset: o.get_field("charset"),
            collation: o.get_field("collation"),
            deletion_policy: o.get_field("deletionPolicy"),
            instance: o.get_field("instance"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            self_link: o.get_field("selfLink"),
        }
    }
}
