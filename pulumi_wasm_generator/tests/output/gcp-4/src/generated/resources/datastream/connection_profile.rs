/// A set of reusable connection configurations to be used as a source or destination for a stream.
///
///
/// To get more information about ConnectionProfile, see:
///
/// * [API documentation](https://cloud.google.com/datastream/docs/reference/rest/v1/projects.locations.connectionProfiles)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/datastream/docs/create-connection-profiles)
///
///
///
/// ## Example Usage
///
/// ### Datastream Connection Profile Basic
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = connection_profile::create(
///         "default",
///         ConnectionProfileArgs::builder()
///             .connection_profile_id("my-profile")
///             .display_name("Connection profile")
///             .gcs_profile(
///                 ConnectionProfileGcsProfile::builder()
///                     .bucket("my-bucket")
///                     .rootPath("/path")
///                     .build_struct(),
///             )
///             .location("us-central1")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Datastream Connection Profile Postgresql Private Connection
///
///
/// ```yaml
/// resources:
///   privateConnection:
///     type: gcp:datastream:PrivateConnection
///     name: private_connection
///     properties:
///       displayName: Connection profile
///       location: us-central1
///       privateConnectionId: my-connection
///       labels:
///         key: value
///       vpcPeeringConfig:
///         vpc: ${default.id}
///         subnet: 10.0.0.0/29
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: my-network
///   instance:
///     type: gcp:sql:DatabaseInstance
///     properties:
///       name: my-instance
///       databaseVersion: POSTGRES_14
///       region: us-central1
///       settings:
///         tier: db-f1-micro
///         ipConfiguration:
///           authorizedNetworks:
///             - value: 34.71.242.81
///             - value: 34.72.28.29
///             - value: 34.67.6.157
///             - value: 34.67.234.134
///             - value: 34.72.239.218
///       deletionProtection: true
///   db:
///     type: gcp:sql:Database
///     properties:
///       instance: ${instance.name}
///       name: db
///   pwd:
///     type: random:RandomPassword
///     properties:
///       length: 16
///       special: false
///   user:
///     type: gcp:sql:User
///     properties:
///       name: user
///       instance: ${instance.name}
///       password: ${pwd.result}
///   defaultConnectionProfile:
///     type: gcp:datastream:ConnectionProfile
///     name: default
///     properties:
///       displayName: Connection profile
///       location: us-central1
///       connectionProfileId: my-profile
///       postgresqlProfile:
///         hostname: ${instance.publicIpAddress}
///         username: ${user.name}
///         password: ${user.password}
///         database: ${db.name}
///       privateConnectivity:
///         privateConnection: ${privateConnection.id}
/// ```
/// ### Datastream Connection Profile Full
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:datastream:ConnectionProfile
///     properties:
///       displayName: Connection profile
///       location: us-central1
///       connectionProfileId: my-profile
///       gcsProfile:
///         bucket: my-bucket
///         rootPath: /path
///       forwardSshConnectivity:
///         hostname: google.com
///         username: my-user
///         port: 8022
///         password: swordfish
///       labels:
///         key: value
/// ```
/// ### Datastream Connection Profile Postgres
///
///
/// ```yaml
/// resources:
///   instance:
///     type: gcp:sql:DatabaseInstance
///     properties:
///       name: my-instance
///       databaseVersion: POSTGRES_14
///       region: us-central1
///       settings:
///         tier: db-f1-micro
///         ipConfiguration:
///           authorizedNetworks:
///             - value: 34.71.242.81
///             - value: 34.72.28.29
///             - value: 34.67.6.157
///             - value: 34.67.234.134
///             - value: 34.72.239.218
///       deletionProtection: true
///   db:
///     type: gcp:sql:Database
///     properties:
///       instance: ${instance.name}
///       name: db
///   pwd:
///     type: random:RandomPassword
///     properties:
///       length: 16
///       special: false
///   user:
///     type: gcp:sql:User
///     properties:
///       name: user
///       instance: ${instance.name}
///       password: ${pwd.result}
///   default:
///     type: gcp:datastream:ConnectionProfile
///     properties:
///       displayName: Connection profile
///       location: us-central1
///       connectionProfileId: my-profile
///       postgresqlProfile:
///         hostname: ${instance.publicIpAddress}
///         username: ${user.name}
///         password: ${user.password}
///         database: ${db.name}
/// ```
/// ### Datastream Connection Profile Sql Server
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let db = database::create(
///         "db",
///         DatabaseArgs::builder().instance("${instance.name}").name("db").build_struct(),
///     );
///     let default = connection_profile::create(
///         "default",
///         ConnectionProfileArgs::builder()
///             .connection_profile_id("source-profile")
///             .display_name("SQL Server Source")
///             .location("us-central1")
///             .sql_server_profile(
///                 ConnectionProfileSqlServerProfile::builder()
///                     .database("${db.name}")
///                     .hostname("${instance.publicIpAddress}")
///                     .password("${user.password}")
///                     .port(1433)
///                     .username("${user.name}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let instance = database_instance::create(
///         "instance",
///         DatabaseInstanceArgs::builder()
///             .database_version("SQLSERVER_2019_STANDARD")
///             .deletion_protection(true)
///             .name("sql-server")
///             .region("us-central1")
///             .root_password("root-password")
///             .settings(
///                 DatabaseInstanceSettings::builder()
///                     .ipConfiguration(
///                         DatabaseInstanceSettingsIpConfiguration::builder()
///                             .authorizedNetworks(
///                                 vec![
///                                     DatabaseInstanceSettingsIpConfigurationAuthorizedNetwork::builder()
///                                     .value("34.71.242.81").build_struct(),
///                                     DatabaseInstanceSettingsIpConfigurationAuthorizedNetwork::builder()
///                                     .value("34.72.28.29").build_struct(),
///                                     DatabaseInstanceSettingsIpConfigurationAuthorizedNetwork::builder()
///                                     .value("34.67.6.157").build_struct(),
///                                     DatabaseInstanceSettingsIpConfigurationAuthorizedNetwork::builder()
///                                     .value("34.67.234.134").build_struct(),
///                                     DatabaseInstanceSettingsIpConfigurationAuthorizedNetwork::builder()
///                                     .value("34.72.239.218").build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .tier("db-custom-2-4096")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let user = user::create(
///         "user",
///         UserArgs::builder()
///             .instance("${instance.name}")
///             .name("user")
///             .password("password")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ConnectionProfile can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/connectionProfiles/{{connection_profile_id}}`
///
/// * `{{project}}/{{location}}/{{connection_profile_id}}`
///
/// * `{{location}}/{{connection_profile_id}}`
///
/// When using the `pulumi import` command, ConnectionProfile can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:datastream/connectionProfile:ConnectionProfile default projects/{{project}}/locations/{{location}}/connectionProfiles/{{connection_profile_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:datastream/connectionProfile:ConnectionProfile default {{project}}/{{location}}/{{connection_profile_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:datastream/connectionProfile:ConnectionProfile default {{location}}/{{connection_profile_id}}
/// ```
///
pub mod connection_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionProfileArgs {
        /// BigQuery warehouse profile.
        #[builder(into, default)]
        pub bigquery_profile: pulumi_wasm_rust::Output<
            Option<super::super::types::datastream::ConnectionProfileBigqueryProfile>,
        >,
        /// The connection profile identifier.
        #[builder(into)]
        pub connection_profile_id: pulumi_wasm_rust::Output<String>,
        /// Create the connection profile without validating it.
        #[builder(into, default)]
        pub create_without_validation: pulumi_wasm_rust::Output<Option<bool>>,
        /// Display name.
        #[builder(into)]
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// Forward SSH tunnel connectivity.
        /// Structure is documented below.
        #[builder(into, default)]
        pub forward_ssh_connectivity: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datastream::ConnectionProfileForwardSshConnectivity,
            >,
        >,
        /// Cloud Storage bucket profile.
        /// Structure is documented below.
        #[builder(into, default)]
        pub gcs_profile: pulumi_wasm_rust::Output<
            Option<super::super::types::datastream::ConnectionProfileGcsProfile>,
        >,
        /// Labels.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the location this connection profile is located in.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// MySQL database profile.
        /// Structure is documented below.
        #[builder(into, default)]
        pub mysql_profile: pulumi_wasm_rust::Output<
            Option<super::super::types::datastream::ConnectionProfileMysqlProfile>,
        >,
        /// Oracle database profile.
        /// Structure is documented below.
        #[builder(into, default)]
        pub oracle_profile: pulumi_wasm_rust::Output<
            Option<super::super::types::datastream::ConnectionProfileOracleProfile>,
        >,
        /// PostgreSQL database profile.
        /// Structure is documented below.
        #[builder(into, default)]
        pub postgresql_profile: pulumi_wasm_rust::Output<
            Option<super::super::types::datastream::ConnectionProfilePostgresqlProfile>,
        >,
        /// Private connectivity.
        /// Structure is documented below.
        #[builder(into, default)]
        pub private_connectivity: pulumi_wasm_rust::Output<
            Option<super::super::types::datastream::ConnectionProfilePrivateConnectivity>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        /// SQL Server database profile.
        /// Structure is documented below.
        #[builder(into, default)]
        pub sql_server_profile: pulumi_wasm_rust::Output<
            Option<super::super::types::datastream::ConnectionProfileSqlServerProfile>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConnectionProfileResult {
        /// BigQuery warehouse profile.
        pub bigquery_profile: pulumi_wasm_rust::Output<
            Option<super::super::types::datastream::ConnectionProfileBigqueryProfile>,
        >,
        /// The connection profile identifier.
        pub connection_profile_id: pulumi_wasm_rust::Output<String>,
        /// Create the connection profile without validating it.
        pub create_without_validation: pulumi_wasm_rust::Output<Option<bool>>,
        /// Display name.
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Forward SSH tunnel connectivity.
        /// Structure is documented below.
        pub forward_ssh_connectivity: pulumi_wasm_rust::Output<
            Option<
                super::super::types::datastream::ConnectionProfileForwardSshConnectivity,
            >,
        >,
        /// Cloud Storage bucket profile.
        /// Structure is documented below.
        pub gcs_profile: pulumi_wasm_rust::Output<
            Option<super::super::types::datastream::ConnectionProfileGcsProfile>,
        >,
        /// Labels.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the location this connection profile is located in.
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// MySQL database profile.
        /// Structure is documented below.
        pub mysql_profile: pulumi_wasm_rust::Output<
            Option<super::super::types::datastream::ConnectionProfileMysqlProfile>,
        >,
        /// The resource's name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Oracle database profile.
        /// Structure is documented below.
        pub oracle_profile: pulumi_wasm_rust::Output<
            Option<super::super::types::datastream::ConnectionProfileOracleProfile>,
        >,
        /// PostgreSQL database profile.
        /// Structure is documented below.
        pub postgresql_profile: pulumi_wasm_rust::Output<
            Option<super::super::types::datastream::ConnectionProfilePostgresqlProfile>,
        >,
        /// Private connectivity.
        /// Structure is documented below.
        pub private_connectivity: pulumi_wasm_rust::Output<
            Option<super::super::types::datastream::ConnectionProfilePrivateConnectivity>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// SQL Server database profile.
        /// Structure is documented below.
        pub sql_server_profile: pulumi_wasm_rust::Output<
            Option<super::super::types::datastream::ConnectionProfileSqlServerProfile>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConnectionProfileArgs) -> ConnectionProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bigquery_profile_binding = args.bigquery_profile.get_inner();
        let connection_profile_id_binding = args.connection_profile_id.get_inner();
        let create_without_validation_binding = args
            .create_without_validation
            .get_inner();
        let display_name_binding = args.display_name.get_inner();
        let forward_ssh_connectivity_binding = args.forward_ssh_connectivity.get_inner();
        let gcs_profile_binding = args.gcs_profile.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let mysql_profile_binding = args.mysql_profile.get_inner();
        let oracle_profile_binding = args.oracle_profile.get_inner();
        let postgresql_profile_binding = args.postgresql_profile.get_inner();
        let private_connectivity_binding = args.private_connectivity.get_inner();
        let project_binding = args.project.get_inner();
        let sql_server_profile_binding = args.sql_server_profile.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:datastream/connectionProfile:ConnectionProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bigqueryProfile".into(),
                    value: &bigquery_profile_binding,
                },
                register_interface::ObjectField {
                    name: "connectionProfileId".into(),
                    value: &connection_profile_id_binding,
                },
                register_interface::ObjectField {
                    name: "createWithoutValidation".into(),
                    value: &create_without_validation_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
                },
                register_interface::ObjectField {
                    name: "forwardSshConnectivity".into(),
                    value: &forward_ssh_connectivity_binding,
                },
                register_interface::ObjectField {
                    name: "gcsProfile".into(),
                    value: &gcs_profile_binding,
                },
                register_interface::ObjectField {
                    name: "labels".into(),
                    value: &labels_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "mysqlProfile".into(),
                    value: &mysql_profile_binding,
                },
                register_interface::ObjectField {
                    name: "oracleProfile".into(),
                    value: &oracle_profile_binding,
                },
                register_interface::ObjectField {
                    name: "postgresqlProfile".into(),
                    value: &postgresql_profile_binding,
                },
                register_interface::ObjectField {
                    name: "privateConnectivity".into(),
                    value: &private_connectivity_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "sqlServerProfile".into(),
                    value: &sql_server_profile_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "bigqueryProfile".into(),
                },
                register_interface::ResultField {
                    name: "connectionProfileId".into(),
                },
                register_interface::ResultField {
                    name: "createWithoutValidation".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "forwardSshConnectivity".into(),
                },
                register_interface::ResultField {
                    name: "gcsProfile".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "mysqlProfile".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "oracleProfile".into(),
                },
                register_interface::ResultField {
                    name: "postgresqlProfile".into(),
                },
                register_interface::ResultField {
                    name: "privateConnectivity".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "sqlServerProfile".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConnectionProfileResult {
            bigquery_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("bigqueryProfile").unwrap(),
            ),
            connection_profile_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionProfileId").unwrap(),
            ),
            create_without_validation: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createWithoutValidation").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            forward_ssh_connectivity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forwardSshConnectivity").unwrap(),
            ),
            gcs_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gcsProfile").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            mysql_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mysqlProfile").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            oracle_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("oracleProfile").unwrap(),
            ),
            postgresql_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("postgresqlProfile").unwrap(),
            ),
            private_connectivity: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("privateConnectivity").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            sql_server_profile: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sqlServerProfile").unwrap(),
            ),
        }
    }
}
