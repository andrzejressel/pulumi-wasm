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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod database {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DatabaseArgs {
        /// The charset value. See MySQL's
        /// [Supported Character Sets and Collations](https://dev.mysql.com/doc/refman/5.7/en/charset-charsets.html)
        /// and Postgres' [Character Set Support](https://www.postgresql.org/docs/9.6/static/multibyte.html)
        /// for more details and supported values. Postgres databases only support
        /// a value of `UTF8` at creation time.
        #[builder(into, default)]
        pub charset: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The collation value. See MySQL's
        /// [Supported Character Sets and Collations](https://dev.mysql.com/doc/refman/5.7/en/charset-charsets.html)
        /// and Postgres' [Collation Support](https://www.postgresql.org/docs/9.6/static/collation.html)
        /// for more details and supported values. Postgres databases only support
        /// a value of `en_US.UTF8` at creation time.
        #[builder(into, default)]
        pub collation: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The deletion policy for the database. Setting ABANDON allows the resource
        /// to be abandoned rather than deleted. This is useful for Postgres, where databases cannot be
        /// deleted from the API if there are users other than cloudsqlsuperuser with access. Possible
        /// values are: "ABANDON", "DELETE". Defaults to "DELETE".
        #[builder(into, default)]
        pub deletion_policy: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Cloud SQL instance. This does not include the project
        /// ID.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub instance: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the database in the Cloud SQL instance.
        /// This does not include the project ID or instance name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct DatabaseResult {
        /// The charset value. See MySQL's
        /// [Supported Character Sets and Collations](https://dev.mysql.com/doc/refman/5.7/en/charset-charsets.html)
        /// and Postgres' [Character Set Support](https://www.postgresql.org/docs/9.6/static/multibyte.html)
        /// for more details and supported values. Postgres databases only support
        /// a value of `UTF8` at creation time.
        pub charset: pulumi_wasm_rust::Output<String>,
        /// The collation value. See MySQL's
        /// [Supported Character Sets and Collations](https://dev.mysql.com/doc/refman/5.7/en/charset-charsets.html)
        /// and Postgres' [Collation Support](https://www.postgresql.org/docs/9.6/static/collation.html)
        /// for more details and supported values. Postgres databases only support
        /// a value of `en_US.UTF8` at creation time.
        pub collation: pulumi_wasm_rust::Output<String>,
        /// The deletion policy for the database. Setting ABANDON allows the resource
        /// to be abandoned rather than deleted. This is useful for Postgres, where databases cannot be
        /// deleted from the API if there are users other than cloudsqlsuperuser with access. Possible
        /// values are: "ABANDON", "DELETE". Defaults to "DELETE".
        pub deletion_policy: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Cloud SQL instance. This does not include the project
        /// ID.
        ///
        ///
        /// - - -
        pub instance: pulumi_wasm_rust::Output<String>,
        /// The name of the database in the Cloud SQL instance.
        /// This does not include the project ID or instance name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The URI of the created resource.
        pub self_link: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DatabaseArgs,
    ) -> DatabaseResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let charset_binding = args.charset.get_output(context).get_inner();
        let collation_binding = args.collation.get_output(context).get_inner();
        let deletion_policy_binding = args
            .deletion_policy
            .get_output(context)
            .get_inner();
        let instance_binding = args.instance.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:sql/database:Database".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "charset".into(),
                    value: &charset_binding,
                },
                register_interface::ObjectField {
                    name: "collation".into(),
                    value: &collation_binding,
                },
                register_interface::ObjectField {
                    name: "deletionPolicy".into(),
                    value: &deletion_policy_binding,
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
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "charset".into(),
                },
                register_interface::ResultField {
                    name: "collation".into(),
                },
                register_interface::ResultField {
                    name: "deletionPolicy".into(),
                },
                register_interface::ResultField {
                    name: "instance".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DatabaseResult {
            charset: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("charset").unwrap(),
            ),
            collation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("collation").unwrap(),
            ),
            deletion_policy: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletionPolicy").unwrap(),
            ),
            instance: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instance").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
        }
    }
}
