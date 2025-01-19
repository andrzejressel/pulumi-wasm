/// A connection profile definition.
///
///
/// To get more information about ConnectionProfile, see:
///
/// * [API documentation](https://cloud.google.com/database-migration/docs/reference/rest/v1/projects.locations.connectionProfiles/create)
/// * How-to Guides
///     * [Database Migration](https://cloud.google.com/database-migration/docs/)
///
///
///
/// ## Example Usage
///
/// ### Database Migration Service Connection Profile Cloudsql
///
///
/// ```yaml
/// resources:
///   cloudsqldb:
///     type: gcp:sql:DatabaseInstance
///     properties:
///       name: my-database
///       databaseVersion: MYSQL_5_7
///       settings:
///         tier: db-n1-standard-1
///         deletionProtectionEnabled: false
///       deletionProtection: false
///   sqlClientCert:
///     type: gcp:sql:SslCert
///     name: sql_client_cert
///     properties:
///       commonName: my-cert
///       instance: ${cloudsqldb.name}
///     options:
///       dependsOn:
///         - ${cloudsqldb}
///   sqldbUser:
///     type: gcp:sql:User
///     name: sqldb_user
///     properties:
///       name: my-username
///       instance: ${cloudsqldb.name}
///       password: my-password
///     options:
///       dependsOn:
///         - ${sqlClientCert}
///   cloudsqlprofile:
///     type: gcp:databasemigrationservice:ConnectionProfile
///     properties:
///       location: us-central1
///       connectionProfileId: my-fromprofileid
///       displayName: my-fromprofileid_display
///       labels:
///         foo: bar
///       mysql:
///         host: ${cloudsqldb.ipAddresses[0].ipAddress}
///         port: 3306
///         username: ${sqldbUser.name}
///         password: ${sqldbUser.password}
///         ssl:
///           clientKey: ${sqlClientCert.privateKey}
///           clientCertificate: ${sqlClientCert.cert}
///           caCertificate: ${sqlClientCert.serverCaCert}
///         cloudSqlId: my-database
///     options:
///       dependsOn:
///         - ${sqldbUser}
///   cloudsqlprofileDestination:
///     type: gcp:databasemigrationservice:ConnectionProfile
///     name: cloudsqlprofile_destination
///     properties:
///       location: us-central1
///       connectionProfileId: my-toprofileid
///       displayName: my-toprofileid_displayname
///       labels:
///         foo: bar
///       cloudsql:
///         settings:
///           databaseVersion: MYSQL_5_7
///           userLabels:
///             cloudfoo: cloudbar
///           tier: db-n1-standard-1
///           edition: ENTERPRISE
///           storageAutoResizeLimit: '0'
///           activationPolicy: ALWAYS
///           ipConfig:
///             enableIpv4: true
///             requireSsl: true
///           autoStorageIncrease: true
///           dataDiskType: PD_HDD
///           dataDiskSizeGb: '11'
///           zone: us-central1-b
///           sourceId: projects/${project.projectId}/locations/us-central1/connectionProfiles/my-fromprofileid
///           rootPassword: testpasscloudsql
///     options:
///       dependsOn:
///         - ${cloudsqlprofile}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Database Migration Service Connection Profile Postgres
///
///
/// ```yaml
/// resources:
///   postgresqldb:
///     type: gcp:sql:DatabaseInstance
///     properties:
///       name: my-database
///       databaseVersion: POSTGRES_12
///       settings:
///         tier: db-custom-2-13312
///       deletionProtection: false
///   sqlClientCert:
///     type: gcp:sql:SslCert
///     name: sql_client_cert
///     properties:
///       commonName: my-cert
///       instance: ${postgresqldb.name}
///     options:
///       dependsOn:
///         - ${postgresqldb}
///   sqldbUser:
///     type: gcp:sql:User
///     name: sqldb_user
///     properties:
///       name: my-username
///       instance: ${postgresqldb.name}
///       password: my-password
///     options:
///       dependsOn:
///         - ${sqlClientCert}
///   postgresprofile:
///     type: gcp:databasemigrationservice:ConnectionProfile
///     properties:
///       location: us-central1
///       connectionProfileId: my-profileid
///       displayName: my-profileid_display
///       labels:
///         foo: bar
///       postgresql:
///         host: ${postgresqldb.ipAddresses[0].ipAddress}
///         port: 5432
///         username: ${sqldbUser.name}
///         password: ${sqldbUser.password}
///         ssl:
///           clientKey: ${sqlClientCert.privateKey}
///           clientCertificate: ${sqlClientCert.cert}
///           caCertificate: ${sqlClientCert.serverCaCert}
///         cloudSqlId: my-database
///     options:
///       dependsOn:
///         - ${sqldbUser}
/// ```
/// ### Database Migration Service Connection Profile Oracle
///
///
/// ```yaml
/// resources:
///   oracleprofile:
///     type: gcp:databasemigrationservice:ConnectionProfile
///     properties:
///       location: us-central1
///       connectionProfileId: my-profileid
///       displayName: my-profileid_display
///       labels:
///         foo: bar
///       oracle:
///         host: host
///         port: 1521
///         username: username
///         password: password
///         databaseService: dbprovider
///         staticServiceIpConnectivity: {}
/// ```
/// ### Database Migration Service Connection Profile Alloydb
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:compute:Network
///     properties:
///       name: vpc-network
///   privateIpAlloc:
///     type: gcp:compute:GlobalAddress
///     name: private_ip_alloc
///     properties:
///       name: private-ip-alloc
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
///   alloydbprofile:
///     type: gcp:databasemigrationservice:ConnectionProfile
///     properties:
///       location: us-central1
///       connectionProfileId: my-profileid
///       displayName: my-profileid_display
///       labels:
///         foo: bar
///       alloydb:
///         clusterId: tf-test-dbmsalloycluster_52865
///         settings:
///           initialUser:
///             user: alloyuser_85840
///             password: alloypass_60302
///           vpcNetwork: ${default.id}
///           labels:
///             alloyfoo: alloybar
///           primaryInstanceSettings:
///             id: priminstid
///             machineConfig:
///               cpuCount: 2
///             databaseFlags: {}
///             labels:
///               alloysinstfoo: allowinstbar
///     options:
///       dependsOn:
///         - ${vpcConnection}
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Database Migration Service Connection Profile Existing Mysql
///
///
/// ```yaml
/// resources:
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
///   existing-mysql:
///     type: gcp:databasemigrationservice:ConnectionProfile
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
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Database Migration Service Connection Profile Existing Postgres
///
///
/// ```yaml
/// resources:
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
///   existing-psql:
///     type: gcp:databasemigrationservice:ConnectionProfile
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
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
/// ### Database Migration Service Connection Profile Existing Alloydb
///
///
/// ```yaml
/// resources:
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
///   existing-alloydb:
///     type: gcp:databasemigrationservice:ConnectionProfile
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
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
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
/// $ pulumi import gcp:databasemigrationservice/connectionProfile:ConnectionProfile default projects/{{project}}/locations/{{location}}/connectionProfiles/{{connection_profile_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:databasemigrationservice/connectionProfile:ConnectionProfile default {{project}}/{{location}}/{{connection_profile_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:databasemigrationservice/connectionProfile:ConnectionProfile default {{location}}/{{connection_profile_id}}
/// ```
///
pub mod connection_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectionProfileArgs {
        /// Specifies required connection parameters, and the parameters required to create an AlloyDB destination cluster.
        /// Structure is documented below.
        #[builder(into, default)]
        pub alloydb: pulumi_wasm_rust::Output<
            Option<
                super::super::types::databasemigrationservice::ConnectionProfileAlloydb,
            >,
        >,
        /// Specifies required connection parameters, and, optionally, the parameters required to create a Cloud SQL destination database instance.
        /// Structure is documented below.
        #[builder(into, default)]
        pub cloudsql: pulumi_wasm_rust::Output<
            Option<
                super::super::types::databasemigrationservice::ConnectionProfileCloudsql,
            >,
        >,
        /// The ID of the connection profile.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub connection_profile_id: pulumi_wasm_rust::Output<String>,
        /// The connection profile display name.
        #[builder(into, default)]
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource labels for connection profile to use to annotate any related underlying resources such as Compute Engine VMs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        #[builder(into, default)]
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the connection profile should reside.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies connection parameters required specifically for MySQL databases.
        /// Structure is documented below.
        #[builder(into, default)]
        pub mysql: pulumi_wasm_rust::Output<
            Option<super::super::types::databasemigrationservice::ConnectionProfileMysql>,
        >,
        /// Specifies connection parameters required specifically for Oracle databases.
        /// Structure is documented below.
        #[builder(into, default)]
        pub oracle: pulumi_wasm_rust::Output<
            Option<
                super::super::types::databasemigrationservice::ConnectionProfileOracle,
            >,
        >,
        /// Specifies connection parameters required specifically for PostgreSQL databases.
        /// Structure is documented below.
        #[builder(into, default)]
        pub postgresql: pulumi_wasm_rust::Output<
            Option<
                super::super::types::databasemigrationservice::ConnectionProfilePostgresql,
            >,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ConnectionProfileResult {
        /// Specifies required connection parameters, and the parameters required to create an AlloyDB destination cluster.
        /// Structure is documented below.
        pub alloydb: pulumi_wasm_rust::Output<
            Option<
                super::super::types::databasemigrationservice::ConnectionProfileAlloydb,
            >,
        >,
        /// Specifies required connection parameters, and, optionally, the parameters required to create a Cloud SQL destination database instance.
        /// Structure is documented below.
        pub cloudsql: pulumi_wasm_rust::Output<
            Option<
                super::super::types::databasemigrationservice::ConnectionProfileCloudsql,
            >,
        >,
        /// The ID of the connection profile.
        ///
        ///
        /// - - -
        pub connection_profile_id: pulumi_wasm_rust::Output<String>,
        /// Output only. The timestamp when the resource was created. A timestamp in RFC3339 UTC 'Zulu' format, accurate to nanoseconds. Example: '2014-10-02T15:01:23.045123456Z'.
        pub create_time: pulumi_wasm_rust::Output<String>,
        /// The database provider.
        pub dbprovider: pulumi_wasm_rust::Output<String>,
        /// The connection profile display name.
        pub display_name: pulumi_wasm_rust::Output<Option<String>>,
        /// All of labels (key/value pairs) present on the resource in GCP, including the labels configured through Pulumi, other clients and services.
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Output only. The error details in case of state FAILED.
        /// Structure is documented below.
        pub errors: pulumi_wasm_rust::Output<
            Vec<super::super::types::databasemigrationservice::ConnectionProfileError>,
        >,
        /// The resource labels for connection profile to use to annotate any related underlying resources such as Compute Engine VMs.
        ///
        /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
        /// Please refer to the field `effective_labels` for all of the labels present on the resource.
        pub labels: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The location where the connection profile should reside.
        pub location: pulumi_wasm_rust::Output<Option<String>>,
        /// Specifies connection parameters required specifically for MySQL databases.
        /// Structure is documented below.
        pub mysql: pulumi_wasm_rust::Output<
            Option<super::super::types::databasemigrationservice::ConnectionProfileMysql>,
        >,
        /// The name of this connection profile resource in the form of projects/{project}/locations/{location}/connectionProfiles/{connectionProfile}.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies connection parameters required specifically for Oracle databases.
        /// Structure is documented below.
        pub oracle: pulumi_wasm_rust::Output<
            Option<
                super::super::types::databasemigrationservice::ConnectionProfileOracle,
            >,
        >,
        /// Specifies connection parameters required specifically for PostgreSQL databases.
        /// Structure is documented below.
        pub postgresql: pulumi_wasm_rust::Output<
            Option<
                super::super::types::databasemigrationservice::ConnectionProfilePostgresql,
            >,
        >,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// The combination of labels configured directly on the resource
        /// and default labels configured on the provider.
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The current connection profile state.
        pub state: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConnectionProfileArgs) -> ConnectionProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let alloydb_binding = args.alloydb.get_inner();
        let cloudsql_binding = args.cloudsql.get_inner();
        let connection_profile_id_binding = args.connection_profile_id.get_inner();
        let display_name_binding = args.display_name.get_inner();
        let labels_binding = args.labels.get_inner();
        let location_binding = args.location.get_inner();
        let mysql_binding = args.mysql.get_inner();
        let oracle_binding = args.oracle.get_inner();
        let postgresql_binding = args.postgresql.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:databasemigrationservice/connectionProfile:ConnectionProfile"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "alloydb".into(),
                    value: &alloydb_binding,
                },
                register_interface::ObjectField {
                    name: "cloudsql".into(),
                    value: &cloudsql_binding,
                },
                register_interface::ObjectField {
                    name: "connectionProfileId".into(),
                    value: &connection_profile_id_binding,
                },
                register_interface::ObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding,
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
                    name: "mysql".into(),
                    value: &mysql_binding,
                },
                register_interface::ObjectField {
                    name: "oracle".into(),
                    value: &oracle_binding,
                },
                register_interface::ObjectField {
                    name: "postgresql".into(),
                    value: &postgresql_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "alloydb".into(),
                },
                register_interface::ResultField {
                    name: "cloudsql".into(),
                },
                register_interface::ResultField {
                    name: "connectionProfileId".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "dbprovider".into(),
                },
                register_interface::ResultField {
                    name: "displayName".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "errors".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "mysql".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "oracle".into(),
                },
                register_interface::ResultField {
                    name: "postgresql".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
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
            alloydb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("alloydb").unwrap(),
            ),
            cloudsql: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("cloudsql").unwrap(),
            ),
            connection_profile_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionProfileId").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            dbprovider: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbprovider").unwrap(),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("displayName").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            errors: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("errors").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            mysql: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mysql").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            oracle: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("oracle").unwrap(),
            ),
            postgresql: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("postgresql").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
        }
    }
}
