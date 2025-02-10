/// A migration job definition.
///
///
/// To get more information about MigrationJob, see:
///
/// * [API documentation](https://cloud.google.com/database-migration/docs/reference/rest/v1/projects.locations.migrationJobs/create)
/// * How-to Guides
///     * [Database Migration](https://cloud.google.com/database-migration/docs/)
///
/// ## Example Usage
///
/// ### Database Migration Service Migration Job Mysql To Mysql
///
///
/// ```yaml
/// resources:
///   sourceCsql:
///     type: gcp:sql:DatabaseInstance
///     name: source_csql
///     properties:
///       name: source-csql
///       databaseVersion: MYSQL_5_7
///       settings:
///         tier: db-n1-standard-1
///         deletionProtectionEnabled: false
///       deletionProtection: false
///   sourceSqlClientCert:
///     type: gcp:sql:SslCert
///     name: source_sql_client_cert
///     properties:
///       commonName: cert
///       instance: ${sourceCsql.name}
///     options:
///       dependsOn:
///         - ${sourceCsql}
///   sourceSqldbUser:
///     type: gcp:sql:User
///     name: source_sqldb_user
///     properties:
///       name: username
///       instance: ${sourceCsql.name}
///       password: password
///     options:
///       dependsOn:
///         - ${sourceSqlClientCert}
///   sourceCp:
///     type: gcp:databasemigrationservice:ConnectionProfile
///     name: source_cp
///     properties:
///       location: us-central1
///       connectionProfileId: source-cp
///       displayName: source-cp_display
///       labels:
///         foo: bar
///       mysql:
///         host: ${sourceCsql.ipAddresses[0].ipAddress}
///         port: 3306
///         username: ${sourceSqldbUser.name}
///         password: ${sourceSqldbUser.password}
///         ssl:
///           clientKey: ${sourceSqlClientCert.privateKey}
///           clientCertificate: ${sourceSqlClientCert.cert}
///           caCertificate: ${sourceSqlClientCert.serverCaCert}
///         cloudSqlId: source-csql
///     options:
///       dependsOn:
///         - ${sourceSqldbUser}
///   destinationCsql:
///     type: gcp:sql:DatabaseInstance
///     name: destination_csql
///     properties:
///       name: destination-csql
///       databaseVersion: MYSQL_5_7
///       settings:
///         tier: db-n1-standard-1
///         deletionProtectionEnabled: false
///       deletionProtection: false
///   destinationCp:
///     type: gcp:databasemigrationservice:ConnectionProfile
///     name: destination_cp
///     properties:
///       location: us-central1
///       connectionProfileId: destination-cp
///       displayName: destination-cp_display
///       labels:
///         foo: bar
///       mysql:
///         cloudSqlId: destination-csql
///     options:
///       dependsOn:
///         - ${destinationCsql}
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: destination-csql
///   mysqltomysql:
///     type: gcp:databasemigrationservice:MigrationJob
///     properties:
///       location: us-central1
///       migrationJobId: my-migrationid
///       displayName: my-migrationid_display
///       labels:
///         foo: bar
///       performanceConfig:
///         dumpParallelLevel: MAX
///       vpcPeeringConnectivity:
///         vpc: ${default.id}
///       dumpType: LOGICAL
///       dumpFlags:
///         dumpFlags:
///           - name: max-allowed-packet
///             value: '1073741824'
///       source: ${sourceCp.name}
///       destination: ${destinationCp.name}
///       type: CONTINUOUS
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Database Migration Service Migration Job Postgres To Postgres
///
///
/// ```yaml
/// resources:
///   sourceCsql:
///     type: gcp:sql:DatabaseInstance
///     name: source_csql
///     properties:
///       name: source-csql
///       databaseVersion: POSTGRES_15
///       settings:
///         tier: db-custom-2-13312
///         deletionProtectionEnabled: false
///       deletionProtection: false
///   sourceSqlClientCert:
///     type: gcp:sql:SslCert
///     name: source_sql_client_cert
///     properties:
///       commonName: cert
///       instance: ${sourceCsql.name}
///     options:
///       dependsOn:
///         - ${sourceCsql}
///   sourceSqldbUser:
///     type: gcp:sql:User
///     name: source_sqldb_user
///     properties:
///       name: username
///       instance: ${sourceCsql.name}
///       password: password
///     options:
///       dependsOn:
///         - ${sourceSqlClientCert}
///   sourceCp:
///     type: gcp:databasemigrationservice:ConnectionProfile
///     name: source_cp
///     properties:
///       location: us-central1
///       connectionProfileId: source-cp
///       displayName: source-cp_display
///       labels:
///         foo: bar
///       postgresql:
///         host: ${sourceCsql.ipAddresses[0].ipAddress}
///         port: 3306
///         username: ${sourceSqldbUser.name}
///         password: ${sourceSqldbUser.password}
///         ssl:
///           clientKey: ${sourceSqlClientCert.privateKey}
///           clientCertificate: ${sourceSqlClientCert.cert}
///           caCertificate: ${sourceSqlClientCert.serverCaCert}
///         cloudSqlId: source-csql
///     options:
///       dependsOn:
///         - ${sourceSqldbUser}
///   destinationCsql:
///     type: gcp:sql:DatabaseInstance
///     name: destination_csql
///     properties:
///       name: destination-csql
///       databaseVersion: POSTGRES_15
///       settings:
///         tier: db-custom-2-13312
///         deletionProtectionEnabled: false
///       deletionProtection: false
///   destinationCp:
///     type: gcp:databasemigrationservice:ConnectionProfile
///     name: destination_cp
///     properties:
///       location: us-central1
///       connectionProfileId: destination-cp
///       displayName: destination-cp_display
///       labels:
///         foo: bar
///       postgresql:
///         cloudSqlId: destination-csql
///     options:
///       dependsOn:
///         - ${destinationCsql}
///   psqltopsql:
///     type: gcp:databasemigrationservice:MigrationJob
///     properties:
///       location: us-central1
///       migrationJobId: my-migrationid
///       displayName: my-migrationid_display
///       labels:
///         foo: bar
///       staticIpConnectivity: {}
///       source: ${sourceCp.name}
///       destination: ${destinationCp.name}
///       type: CONTINUOUS
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Database Migration Service Migration Job Postgres To Alloydb
///
///
/// ```yaml
/// resources:
///   sourceCsql:
///     type: gcp:sql:DatabaseInstance
///     name: source_csql
///     properties:
///       name: source-csql
///       databaseVersion: POSTGRES_15
///       settings:
///         tier: db-custom-2-13312
///         deletionProtectionEnabled: false
///       deletionProtection: false
///   sourceSqlClientCert:
///     type: gcp:sql:SslCert
///     name: source_sql_client_cert
///     properties:
///       commonName: cert
///       instance: ${sourceCsql.name}
///     options:
///       dependsOn:
///         - ${sourceCsql}
///   sourceSqldbUser:
///     type: gcp:sql:User
///     name: source_sqldb_user
///     properties:
///       name: username
///       instance: ${sourceCsql.name}
///       password: password
///     options:
///       dependsOn:
///         - ${sourceSqlClientCert}
///   sourceCp:
///     type: gcp:databasemigrationservice:ConnectionProfile
///     name: source_cp
///     properties:
///       location: us-central1
///       connectionProfileId: source-cp
///       displayName: source-cp_display
///       labels:
///         foo: bar
///       postgresql:
///         host: ${sourceCsql.ipAddresses[0].ipAddress}
///         port: 3306
///         username: ${sourceSqldbUser.name}
///         password: ${sourceSqldbUser.password}
///         ssl:
///           clientKey: ${sourceSqlClientCert.privateKey}
///           clientCertificate: ${sourceSqlClientCert.cert}
///           caCertificate: ${sourceSqlClientCert.serverCaCert}
///         cloudSqlId: source-csql
///     options:
///       dependsOn:
///         - ${sourceSqldbUser}
///   destinationAlloydb:
///     type: gcp:alloydb:Cluster
///     name: destination_alloydb
///     properties:
///       clusterId: destination-alloydb
///       location: us-central1
///       networkConfig:
///         network: ${default.id}
///       databaseVersion: POSTGRES_15
///       initialUser:
///         user: destination-alloydb
///         password: destination-alloydb
///   destinationAlloydbPrimary:
///     type: gcp:alloydb:Instance
///     name: destination_alloydb_primary
///     properties:
///       cluster: ${destinationAlloydb.name}
///       instanceId: destination-alloydb-primary
///       instanceType: PRIMARY
///     options:
///       dependsOn:
///         - ${vpcConnection}
///   privateIpAlloc:
///     type: gcp:compute:GlobalAddress
///     name: private_ip_alloc
///     properties:
///       name: destination-alloydb
///       addressType: INTERNAL
///       purpose: VPC_PEERING
///       prefixLength: 16
///       network: ${default.id}
///   vpcConnection:
///     type: gcp:servicenetworking:Connection
///     name: vpc_connection
///     properties:
///       network: ${default.id}
///       service: servicenetworking.googleapis.com
///       reservedPeeringRanges:
///         - ${privateIpAlloc.name}
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: destination-alloydb
///   destinationCp:
///     type: gcp:databasemigrationservice:ConnectionProfile
///     name: destination_cp
///     properties:
///       location: us-central1
///       connectionProfileId: destination-cp
///       displayName: destination-cp_display
///       labels:
///         foo: bar
///       postgresql:
///         alloydbClusterId: destination-alloydb
///     options:
///       dependsOn:
///         - ${destinationAlloydb}
///         - ${destinationAlloydbPrimary}
///   psqltoalloydb:
///     type: gcp:databasemigrationservice:MigrationJob
///     properties:
///       location: us-central1
///       migrationJobId: my-migrationid
///       displayName: my-migrationid_display
///       labels:
///         foo: bar
///       staticIpConnectivity: {}
///       source: ${sourceCp.name}
///       destination: ${destinationCp.name}
///       type: CONTINUOUS
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// MigrationJob can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/migrationJobs/{{migration_job_id}}`
///
/// * `{{project}}/{{location}}/{{migration_job_id}}`
///
/// * `{{location}}/{{migration_job_id}}`
///
/// When using the `pulumi import` command, MigrationJob can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:databasemigrationservice/migrationJob:MigrationJob default projects/{{project}}/locations/{{location}}/migrationJobs/{{migration_job_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:databasemigrationservice/migrationJob:MigrationJob default {{project}}/{{location}}/{{migration_job_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:databasemigrationservice/migrationJob:MigrationJob default {{location}}/{{migration_job_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod migration_job {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MigrationJobArgs {
        /// The name of the destination connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{destinationConnectionProfile}.
        #[builder(into)]
        pub destination: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The migration job display name.
        #[builder(into, default)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The initial dump flags.
        /// Structure is documented below.
        #[builder(into, default)]
        pub dump_flags: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::databasemigrationservice::MigrationJobDumpFlags>,
        >,
        /// The path to the dump file in Google Cloud Storage,
        /// in the format: (gs://[BUCKET_NAME]/[OBJECT_NAME]).
        /// This field and the "dump_flags" field are mutually exclusive.
        #[builder(into, default)]
        pub dump_path: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of the data dump. Supported for MySQL to CloudSQL for MySQL
        /// migrations only.
        /// Possible values are: `LOGICAL`, `PHYSICAL`.
        #[builder(into, default)]
        pub dump_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource labels for migration job to use to annotate any related underlying resources such as Compute Engine VMs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the migration job should reside.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the migration job.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub migration_job_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Data dump parallelism settings used by the migration.
        /// Structure is documented below.
        #[builder(into, default)]
        pub performance_config: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::databasemigrationservice::MigrationJobPerformanceConfig,
            >,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The details of the VPC network that the source database is located in.
        /// Structure is documented below.
        #[builder(into, default)]
        pub reverse_ssh_connectivity: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::databasemigrationservice::MigrationJobReverseSshConnectivity,
            >,
        >,
        /// The name of the source connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{sourceConnectionProfile}.
        #[builder(into)]
        pub source: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If set to an empty object (`{}`), the source database will allow incoming
        /// connections from the public IP of the destination database.
        /// You can retrieve the public IP of the Cloud SQL instance from the
        /// Cloud SQL console or using Cloud SQL APIs.
        #[builder(into, default)]
        pub static_ip_connectivity: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::databasemigrationservice::MigrationJobStaticIpConnectivity,
            >,
        >,
        /// The type of the migration job.
        /// Possible values are: `ONE_TIME`, `CONTINUOUS`.
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The details of the VPC network that the source database is located in.
        /// Structure is documented below.
        #[builder(into, default)]
        pub vpc_peering_connectivity: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::databasemigrationservice::MigrationJobVpcPeeringConnectivity,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct MigrationJobResult {
        /// Output only. The timestamp when the resource was created. A timestamp in RFC3339 UTC 'Zulu' format, accurate to nanoseconds. Example: '2014-10-02T15:01:23.045123456Z'.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The name of the destination connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{destinationConnectionProfile}.
        pub destination: pulumi_gestalt_rust::Output<String>,
        /// The migration job display name.
        pub display_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The initial dump flags.
        /// Structure is documented below.
        pub dump_flags: pulumi_gestalt_rust::Output<
            Option<super::super::types::databasemigrationservice::MigrationJobDumpFlags>,
        >,
        /// The path to the dump file in Google Cloud Storage,
        /// in the format: (gs://[BUCKET_NAME]/[OBJECT_NAME]).
        /// This field and the "dump_flags" field are mutually exclusive.
        pub dump_path: pulumi_gestalt_rust::Output<Option<String>>,
        /// The type of the data dump. Supported for MySQL to CloudSQL for MySQL
        /// migrations only.
        /// Possible values are: `LOGICAL`, `PHYSICAL`.
        pub dump_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. The error details in case of state FAILED.
        /// Structure is documented below.
        pub errors: pulumi_gestalt_rust::Output<
            Vec<super::super::types::databasemigrationservice::MigrationJobError>,
        >,
        /// The resource labels for migration job to use to annotate any related underlying resources such as Compute Engine VMs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the migration job should reside.
        pub location: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the migration job.
        ///
        ///
        /// - - -
        pub migration_job_id: pulumi_gestalt_rust::Output<String>,
        /// The name of this migration job resource in the form of projects/{project}/locations/{location}/migrationJobs/{migrationJob}.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Data dump parallelism settings used by the migration.
        /// Structure is documented below.
        pub performance_config: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::databasemigrationservice::MigrationJobPerformanceConfig,
            >,
        >,
        /// The current migration job phase.
        pub phase: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The details of the VPC network that the source database is located in.
        /// Structure is documented below.
        pub reverse_ssh_connectivity: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::databasemigrationservice::MigrationJobReverseSshConnectivity,
            >,
        >,
        /// The name of the source connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{sourceConnectionProfile}.
        pub source: pulumi_gestalt_rust::Output<String>,
        /// The current migration job state.
        pub state: pulumi_gestalt_rust::Output<String>,
        /// If set to an empty object (`{}`), the source database will allow incoming
        /// connections from the public IP of the destination database.
        /// You can retrieve the public IP of the Cloud SQL instance from the
        /// Cloud SQL console or using Cloud SQL APIs.
        pub static_ip_connectivity: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::databasemigrationservice::MigrationJobStaticIpConnectivity,
            >,
        >,
        /// The type of the migration job.
        /// Possible values are: `ONE_TIME`, `CONTINUOUS`.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The details of the VPC network that the source database is located in.
        /// Structure is documented below.
        pub vpc_peering_connectivity: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::databasemigrationservice::MigrationJobVpcPeeringConnectivity,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MigrationJobArgs,
    ) -> MigrationJobResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let destination_binding = args.destination.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let dump_flags_binding = args.dump_flags.get_output(context);
        let dump_path_binding = args.dump_path.get_output(context);
        let dump_type_binding = args.dump_type.get_output(context);
        let labels_binding = args.labels.get_output(context);
        let location_binding = args.location.get_output(context);
        let migration_job_id_binding = args.migration_job_id.get_output(context);
        let performance_config_binding = args.performance_config.get_output(context);
        let project_binding = args.project.get_output(context);
        let reverse_ssh_connectivity_binding = args
            .reverse_ssh_connectivity
            .get_output(context);
        let source_binding = args.source.get_output(context);
        let static_ip_connectivity_binding = args
            .static_ip_connectivity
            .get_output(context);
        let type__binding = args.type_.get_output(context);
        let vpc_peering_connectivity_binding = args
            .vpc_peering_connectivity
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:databasemigrationservice/migrationJob:MigrationJob".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destination".into(),
                    value: destination_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: display_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dumpFlags".into(),
                    value: dump_flags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dumpPath".into(),
                    value: dump_path_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dumpType".into(),
                    value: dump_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labels".into(),
                    value: labels_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "migrationJobId".into(),
                    value: migration_job_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "performanceConfig".into(),
                    value: performance_config_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "reverseSshConnectivity".into(),
                    value: reverse_ssh_connectivity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "source".into(),
                    value: source_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "staticIpConnectivity".into(),
                    value: static_ip_connectivity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: type__binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpcPeeringConnectivity".into(),
                    value: vpc_peering_connectivity_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MigrationJobResult {
            create_time: o.get_field("createTime"),
            destination: o.get_field("destination"),
            display_name: o.get_field("displayName"),
            dump_flags: o.get_field("dumpFlags"),
            dump_path: o.get_field("dumpPath"),
            dump_type: o.get_field("dumpType"),
            effective_labels: o.get_field("effectiveLabels"),
            errors: o.get_field("errors"),
            labels: o.get_field("labels"),
            location: o.get_field("location"),
            migration_job_id: o.get_field("migrationJobId"),
            name: o.get_field("name"),
            performance_config: o.get_field("performanceConfig"),
            phase: o.get_field("phase"),
            project: o.get_field("project"),
            pulumi_labels: o.get_field("pulumiLabels"),
            reverse_ssh_connectivity: o.get_field("reverseSshConnectivity"),
            source: o.get_field("source"),
            state: o.get_field("state"),
            static_ip_connectivity: o.get_field("staticIpConnectivity"),
            type_: o.get_field("type"),
            vpc_peering_connectivity: o.get_field("vpcPeeringConnectivity"),
        }
    }
}
