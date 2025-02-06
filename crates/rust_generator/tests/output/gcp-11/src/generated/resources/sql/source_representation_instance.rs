/// A source representation instance is a Cloud SQL instance that represents
/// the source database server to the Cloud SQL replica. It is visible in the
/// Cloud Console and appears the same as a regular Cloud SQL instance, but it
/// contains no data, requires no configuration or maintenance, and does not
/// affect billing. You cannot update the source representation instance.
///
///
///
///
///
/// ## Example Usage
///
/// ### Sql Source Representation Instance Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let instance = source_representation_instance::create(
///         "instance",
///         SourceRepresentationInstanceArgs::builder()
///             .database_version("MYSQL_8_0")
///             .dump_file_path("gs://replica-bucket/source-database.sql.gz")
///             .host("10.20.30.40")
///             .name("my-instance")
///             .password("password-for-the-user")
///             .port(3306)
///             .region("us-central1")
///             .username("some-user")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Sql Source Representation Instance Postgres
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let instance = source_representation_instance::create(
///         "instance",
///         SourceRepresentationInstanceArgs::builder()
///             .database_version("POSTGRES_9_6")
///             .dump_file_path("gs://replica-bucket/source-database.sql.gz")
///             .host("10.20.30.40")
///             .name("my-instance")
///             .password("password-for-the-user")
///             .port(3306)
///             .region("us-central1")
///             .username("some-user")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// SourceRepresentationInstance can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/instances/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, SourceRepresentationInstance can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:sql/sourceRepresentationInstance:SourceRepresentationInstance default projects/{{project}}/instances/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:sql/sourceRepresentationInstance:SourceRepresentationInstance default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:sql/sourceRepresentationInstance:SourceRepresentationInstance default {{name}}
/// ```
///
pub mod source_representation_instance {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SourceRepresentationInstanceArgs {
        /// The CA certificate on the external server. Include only if SSL/TLS is used on the external server.
        #[builder(into, default)]
        pub ca_certificate: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The client certificate on the external server. Required only for server-client authentication. Include only if SSL/TLS is used on the external server.
        #[builder(into, default)]
        pub client_certificate: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The private key file for the client certificate on the external server. Required only for server-client authentication. Include only if SSL/TLS is used on the external server.
        #[builder(into, default)]
        pub client_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The MySQL version running on your source database server.
        /// Possible values are: `MYSQL_5_6`, `MYSQL_5_7`, `MYSQL_8_0`, `POSTGRES_9_6`, `POSTGRES_10`, `POSTGRES_11`, `POSTGRES_12`, `POSTGRES_13`, `POSTGRES_14`.
        #[builder(into)]
        pub database_version: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A file in the bucket that contains the data from the external server.
        #[builder(into, default)]
        pub dump_file_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IPv4 address and port for the external server, or the the DNS address for the external server. If the external server is hosted on Cloud SQL, the port is 5432.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub host: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the source representation instance. Use any valid Cloud SQL instance name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The password for the replication user account.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        #[builder(into, default)]
        pub password: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The externally accessible port for the source database server.
        /// Defaults to 3306.
        #[builder(into, default)]
        pub port: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Region in which the created instance should reside.
        /// If it is not provided, the provider region is used.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The replication user account on the external server.
        #[builder(into, default)]
        pub username: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SourceRepresentationInstanceResult {
        /// The CA certificate on the external server. Include only if SSL/TLS is used on the external server.
        pub ca_certificate: pulumi_gestalt_rust::Output<Option<String>>,
        /// The client certificate on the external server. Required only for server-client authentication. Include only if SSL/TLS is used on the external server.
        pub client_certificate: pulumi_gestalt_rust::Output<Option<String>>,
        /// The private key file for the client certificate on the external server. Required only for server-client authentication. Include only if SSL/TLS is used on the external server.
        pub client_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// The MySQL version running on your source database server.
        /// Possible values are: `MYSQL_5_6`, `MYSQL_5_7`, `MYSQL_8_0`, `POSTGRES_9_6`, `POSTGRES_10`, `POSTGRES_11`, `POSTGRES_12`, `POSTGRES_13`, `POSTGRES_14`.
        pub database_version: pulumi_gestalt_rust::Output<String>,
        /// A file in the bucket that contains the data from the external server.
        pub dump_file_path: pulumi_gestalt_rust::Output<Option<String>>,
        /// The IPv4 address and port for the external server, or the the DNS address for the external server. If the external server is hosted on Cloud SQL, the port is 5432.
        ///
        ///
        /// - - -
        pub host: pulumi_gestalt_rust::Output<String>,
        /// The name of the source representation instance. Use any valid Cloud SQL instance name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The password for the replication user account.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        pub password: pulumi_gestalt_rust::Output<Option<String>>,
        /// The externally accessible port for the source database server.
        /// Defaults to 3306.
        pub port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The Region in which the created instance should reside.
        /// If it is not provided, the provider region is used.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The replication user account on the external server.
        pub username: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: SourceRepresentationInstanceArgs,
    ) -> SourceRepresentationInstanceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let ca_certificate_binding = args.ca_certificate.get_output(context).get_inner();
        let client_certificate_binding = args
            .client_certificate
            .get_output(context)
            .get_inner();
        let client_key_binding = args.client_key.get_output(context).get_inner();
        let database_version_binding = args
            .database_version
            .get_output(context)
            .get_inner();
        let dump_file_path_binding = args.dump_file_path.get_output(context).get_inner();
        let host_binding = args.host.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let password_binding = args.password.get_output(context).get_inner();
        let port_binding = args.port.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let region_binding = args.region.get_output(context).get_inner();
        let username_binding = args.username.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:sql/sourceRepresentationInstance:SourceRepresentationInstance"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "caCertificate".into(),
                    value: &ca_certificate_binding,
                },
                register_interface::ObjectField {
                    name: "clientCertificate".into(),
                    value: &client_certificate_binding,
                },
                register_interface::ObjectField {
                    name: "clientKey".into(),
                    value: &client_key_binding,
                },
                register_interface::ObjectField {
                    name: "databaseVersion".into(),
                    value: &database_version_binding,
                },
                register_interface::ObjectField {
                    name: "dumpFilePath".into(),
                    value: &dump_file_path_binding,
                },
                register_interface::ObjectField {
                    name: "host".into(),
                    value: &host_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "password".into(),
                    value: &password_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
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
                    name: "username".into(),
                    value: &username_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        SourceRepresentationInstanceResult {
            ca_certificate: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("caCertificate"),
            ),
            client_certificate: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientCertificate"),
            ),
            client_key: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("clientKey"),
            ),
            database_version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("databaseVersion"),
            ),
            dump_file_path: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dumpFilePath"),
            ),
            host: pulumi_gestalt_rust::__private::into_domain(o.extract_field("host")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            password: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("password"),
            ),
            port: pulumi_gestalt_rust::__private::into_domain(o.extract_field("port")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            region: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("region"),
            ),
            username: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("username"),
            ),
        }
    }
}
