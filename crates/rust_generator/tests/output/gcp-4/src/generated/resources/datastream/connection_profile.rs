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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod connection_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionProfileArgs {
        /// BigQuery warehouse profile.
        #[builder(into, default)]
        pub bigquery_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datastream::ConnectionProfileBigqueryProfile>,
        >,
        /// The connection profile identifier.
        #[builder(into)]
        pub connection_profile_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Create the connection profile without validating it.
        #[builder(into, default)]
        pub create_without_validation: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Display name.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Forward SSH tunnel connectivity.
        /// Structure is documented below.
        #[builder(into, default)]
        pub forward_ssh_connectivity: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::datastream::ConnectionProfileForwardSshConnectivity,
            >,
        >,
        /// Cloud Storage bucket profile.
        /// Structure is documented below.
        #[builder(into, default)]
        pub gcs_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datastream::ConnectionProfileGcsProfile>,
        >,
        /// Labels.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the location this connection profile is located in.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// MySQL database profile.
        /// Structure is documented below.
        #[builder(into, default)]
        pub mysql_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datastream::ConnectionProfileMysqlProfile>,
        >,
        /// Oracle database profile.
        /// Structure is documented below.
        #[builder(into, default)]
        pub oracle_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datastream::ConnectionProfileOracleProfile>,
        >,
        /// PostgreSQL database profile.
        /// Structure is documented below.
        #[builder(into, default)]
        pub postgresql_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datastream::ConnectionProfilePostgresqlProfile>,
        >,
        /// Private connectivity.
        /// Structure is documented below.
        #[builder(into, default)]
        pub private_connectivity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datastream::ConnectionProfilePrivateConnectivity>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// SQL Server database profile.
        /// Structure is documented below.
        #[builder(into, default)]
        pub sql_server_profile: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::datastream::ConnectionProfileSqlServerProfile>,
        >,
    }
    #[allow(dead_code)]
    pub struct ConnectionProfileResult {
        /// BigQuery warehouse profile.
        pub bigquery_profile: pulumi_gestalt_rust::Output<
            Option<super::super::types::datastream::ConnectionProfileBigqueryProfile>,
        >,
        /// The connection profile identifier.
        pub connection_profile_id: pulumi_gestalt_rust::Output<String>,
        /// Create the connection profile without validating it.
        pub create_without_validation: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Display name.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Forward SSH tunnel connectivity.
        /// Structure is documented below.
        pub forward_ssh_connectivity: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::datastream::ConnectionProfileForwardSshConnectivity,
            >,
        >,
        /// Cloud Storage bucket profile.
        /// Structure is documented below.
        pub gcs_profile: pulumi_gestalt_rust::Output<
            Option<super::super::types::datastream::ConnectionProfileGcsProfile>,
        >,
        /// Labels.
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The name of the location this connection profile is located in.
        ///
        ///
        /// - - -
        pub location: pulumi_gestalt_rust::Output<String>,
        /// MySQL database profile.
        /// Structure is documented below.
        pub mysql_profile: pulumi_gestalt_rust::Output<
            Option<super::super::types::datastream::ConnectionProfileMysqlProfile>,
        >,
        /// The resource's name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Oracle database profile.
        /// Structure is documented below.
        pub oracle_profile: pulumi_gestalt_rust::Output<
            Option<super::super::types::datastream::ConnectionProfileOracleProfile>,
        >,
        /// PostgreSQL database profile.
        /// Structure is documented below.
        pub postgresql_profile: pulumi_gestalt_rust::Output<
            Option<super::super::types::datastream::ConnectionProfilePostgresqlProfile>,
        >,
        /// Private connectivity.
        /// Structure is documented below.
        pub private_connectivity: pulumi_gestalt_rust::Output<
            Option<super::super::types::datastream::ConnectionProfilePrivateConnectivity>,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// SQL Server database profile.
        /// Structure is documented below.
        pub sql_server_profile: pulumi_gestalt_rust::Output<
            Option<super::super::types::datastream::ConnectionProfileSqlServerProfile>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConnectionProfileArgs,
    ) -> ConnectionProfileResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bigquery_profile_binding = args.bigquery_profile.get_output(context);
        let connection_profile_id_binding = args
            .connection_profile_id
            .get_output(context);
        let create_without_validation_binding = args
            .create_without_validation
            .get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let forward_ssh_connectivity_binding = args
            .forward_ssh_connectivity
            .get_output(context);
        let gcs_profile_binding = args.gcs_profile.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let mysql_profile_binding = args.mysql_profile.get_output(context);
        let oracle_profile_binding = args.oracle_profile.get_output(context);
        let postgresql_profile_binding = args.postgresql_profile.get_output(context);
        let private_connectivity_binding = args.private_connectivity.get_output(context);
        let project_binding = args.project.get_output(context);
        let sql_server_profile_binding = args.sql_server_profile.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:datastream/connectionProfile:ConnectionProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bigqueryProfile".into(),
                    value: &bigquery_profile_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionProfileId".into(),
                    value: &connection_profile_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "createWithoutValidation".into(),
                    value: &create_without_validation_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "forwardSshConnectivity".into(),
                    value: &forward_ssh_connectivity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gcsProfile".into(),
                    value: &gcs_profile_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: &labels_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mysqlProfile".into(),
                    value: &mysql_profile_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "oracleProfile".into(),
                    value: &oracle_profile_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "postgresqlProfile".into(),
                    value: &postgresql_profile_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "privateConnectivity".into(),
                    value: &private_connectivity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sqlServerProfile".into(),
                    value: &sql_server_profile_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConnectionProfileResult {
            bigquery_profile: o.get_field("bigqueryProfile"),
            connection_profile_id: o.get_field("connectionProfileId"),
            create_without_validation: o.get_field("createWithoutValidation"),
            display_name: o.get_field("displayName"),
            effective_labels: o.get_field("effectiveLabels"),
            forward_ssh_connectivity: o.get_field("forwardSshConnectivity"),
            gcs_profile: o.get_field("gcsProfile"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            mysql_profile: o.get_field("mysqlProfile"),
            name: o.get_field("name"),
            oracle_profile: o.get_field("oracleProfile"),
            postgresql_profile: o.get_field("postgresqlProfile"),
            private_connectivity: o.get_field("privateConnectivity"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            sql_server_profile: o.get_field("sqlServerProfile"),
        }
    }
}
